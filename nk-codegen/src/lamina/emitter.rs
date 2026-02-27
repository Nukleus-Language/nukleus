use std::collections::HashMap;

use astgen::ast::{ASTOperator, ASTstatement, ASTtypecomp, ASTtypename, ASTtypevalue, AST};

use crate::error::CodegenError;

use super::helpers::{
    choose_binary_type, escape_char, extract_identifier,
    extract_identifier_from_ast, infer_type_from_typevalue, lamina_binary_op, lamina_cmp_op,
    lamina_type, FunctionSignature, LoweredValue,
};

pub(super) struct FunctionEmitter<'a> {
    signatures: &'a HashMap<String, FunctionSignature>,
    lines: Vec<String>,
    vars: HashMap<String, ASTtypename>,
    var_ptrs: HashMap<String, String>,
    temp_counter: usize,
    label_counter: usize,
    terminated: bool,
}

impl<'a> FunctionEmitter<'a> {
    pub(super) fn new(signatures: &'a HashMap<String, FunctionSignature>) -> Self {
        Self {
            signatures,
            lines: Vec::new(),
            vars: HashMap::new(),
            var_ptrs: HashMap::new(),
            temp_counter: 0,
            label_counter: 0,
            terminated: false,
        }
    }

    pub(super) fn lower_function(
        &mut self,
        name: &str,
        args: &[ASTtypecomp],
        statements: &[AST],
        return_type: ASTtypename,
    ) -> Result<String, CodegenError> {
        let args_text = self.lower_params(args)?;
        let ret_text = lamina_type(return_type)?;

        let mut out = String::new();
        out.push_str(&format!("fn @{}({}) -> {} {{\n", name, args_text, ret_text));
        out.push_str("  entry:\n");

        for arg in args {
            if let ASTtypecomp::Argument {
                type_name,
                identifier,
            } = arg
            {
                let arg_name = extract_identifier(identifier)?;
                let ptr_name = format!("{}_ptr", arg_name);
                let ty_name = lamina_type(*type_name)?;
                self.emit_inst(format!("%{} = alloc.stack {}", ptr_name, ty_name));
                self.emit_inst(format!("store.{} %{}, %{}", ty_name, ptr_name, arg_name));
                self.vars.insert(arg_name.to_string(), *type_name);
                self.var_ptrs.insert(arg_name.to_string(), ptr_name);
            }
        }

        self.lower_statements(statements)?;
        if !self.terminated {
            self.emit_default_return(return_type)?;
        }

        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out.push('}');
        Ok(out)
    }

    fn lower_params(&self, args: &[ASTtypecomp]) -> Result<String, CodegenError> {
        let mut params = Vec::new();
        for arg in args {
            if let ASTtypecomp::Argument {
                type_name,
                identifier,
            } = arg
            {
                params.push(format!(
                    "{} %{}",
                    lamina_type(*type_name)?,
                    extract_identifier(identifier)?
                ));
            }
        }
        Ok(params.join(", "))
    }

    fn lower_print(
        &mut self,
        value: &AST,
        args: &[AST],
        add_newline: bool,
    ) -> Result<(), CodegenError> {
        if !args.is_empty() {
            return Err(CodegenError::CompilationError(
                "Format string arguments not yet supported for print/println".to_string(),
            ));
        }

        if let AST::TypeValue(astgen::ast::ASTtypevalue::QuotedString(s)) = value {
            for b in s.bytes() {
                let code = i64::from(b);
                let byte_tmp = self.new_temp();
                let discard = self.new_temp();
                self.emit_inst(format!("%{} = add.i64 {}, 0", byte_tmp, code));
                self.emit_inst(format!("%{} = writebyte %{}", discard, byte_tmp));
            }
            if add_newline {
                let discard = self.new_temp();
                self.emit_inst(format!("%{} = writebyte 10", discard));
            }
            return Ok(());
        }

        let lowered = self.lower_expr(value)?;
        let print_var = if lowered.repr.starts_with('%') {
            lowered.repr
        } else {
            let tmp = self.new_temp();
            self.emit_inst(format!(
                "%{} = add.{} {}, 0",
                tmp,
                super::helpers::lamina_type(lowered.ty)?,
                lowered.repr
            ));
            format!("%{}", tmp)
        };
        self.emit_inst(format!("print {}", print_var));
        if add_newline {
            let discard = self.new_temp();
            self.emit_inst(format!("%{} = writebyte 10", discard));
        }
        Ok(())
    }

    fn lower_statements(&mut self, statements: &[AST]) -> Result<(), CodegenError> {
        for stmt in statements {
            if self.terminated {
                break;
            }
            self.lower_statement(stmt)?;
        }
        Ok(())
    }

    fn lower_statement(&mut self, stmt: &AST) -> Result<(), CodegenError> {
        match stmt {
            AST::Statement(ASTstatement::Let {
                name,
                type_name,
                value,
            }) => {
                let lowered = match value {
                    Some(expr) => self.lower_expr(expr)?,
                    None => LoweredValue {
                        repr: "0".to_string(),
                        ty: type_name.unwrap_or(ASTtypename::I64),
                    },
                };

                let ty = type_name.unwrap_or(lowered.ty);
                let ptr_name = format!("{}_ptr", name);
                let ty_name = lamina_type(ty)?;

                self.emit_inst(format!("%{} = alloc.stack {}", ptr_name, ty_name));
                self.emit_inst(format!("store.{} %{}, {}", ty_name, ptr_name, lowered.repr));
                self.vars.insert(name.clone(), ty);
                self.var_ptrs.insert(name.clone(), ptr_name);
            }
            AST::Statement(ASTstatement::Assignment { left, op, right }) => {
                let var_name = extract_identifier_from_ast(left)?;
                let var_ty = *self.vars.get(var_name).ok_or_else(|| {
                    CodegenError::VariableNotFound(format!("Unknown variable '{}'", var_name))
                })?;
                let ptr_name = self.var_ptrs.get(var_name).cloned().ok_or_else(|| {
                    CodegenError::VariableNotFound(format!("Unknown variable '{}'", var_name))
                })?;
                let ty_name = lamina_type(var_ty)?;
                let rhs = self.lower_expr(right)?;

                match op {
                    ASTOperator::Assign => {
                        self.emit_inst(format!("store.{} %{}, {}", ty_name, ptr_name, rhs.repr));
                    }
                    _ => {
                        let current = self.load_variable(var_name, var_ty)?;
                        let result = self.new_temp();
                        self.emit_inst(format!(
                            "%{} = {}.{} {}, {}",
                            result,
                            lamina_binary_op(op.clone())?,
                            ty_name,
                            current.repr,
                            rhs.repr
                        ));
                        self.emit_inst(format!("store.{} %{}, %{}", ty_name, ptr_name, result));
                    }
                }
            }
            AST::Statement(ASTstatement::Print { value, args })
            | AST::Statement(ASTstatement::Println { value, args }) => {
                self.lower_print(value, args, matches!(stmt, AST::Statement(ASTstatement::Println { .. })))?;
            }
            AST::Statement(ASTstatement::Return { value }) => {
                let lowered = self.lower_expr(value)?;
                self.emit_terminator(format!("ret.{} {}", lamina_type(lowered.ty)?, lowered.repr));
            }
            AST::Statement(ASTstatement::If {
                condition,
                statements,
                elif,
                else_statements,
            }) => {
                let merge_label = self.new_label("if_merge");
                self.lower_if_chain(
                    condition,
                    statements,
                    elif.as_deref(),
                    else_statements.as_ref(),
                    &merge_label,
                )?;
                self.emit_label(&merge_label);
            }
            AST::Statement(ASTstatement::For {
                start,
                end,
                value,
                statements,
            }) => {
                let loop_var_name = match start {
                    ASTtypevalue::Identifier(name) => name,
                    _ => {
                        return Err(CodegenError::CompilationError(
                            "for-loop start must be an identifier".to_string(),
                        ));
                    }
                };

                let loop_ty = *self.vars.get(loop_var_name).ok_or_else(|| {
                    CodegenError::VariableNotFound(format!(
                        "Unknown loop variable '{}'",
                        loop_var_name
                    ))
                })?;
                let ptr_name = self.var_ptrs.get(loop_var_name).cloned().ok_or_else(|| {
                    CodegenError::VariableNotFound(format!(
                        "Unknown loop variable '{}'",
                        loop_var_name
                    ))
                })?;

                let cond_label = self.new_label("for_cond");
                let body_label = self.new_label("for_body");
                let end_label = self.new_label("for_end");
                let ty_name = lamina_type(loop_ty)?;

                self.emit_terminator(format!("jmp {}", cond_label));

                self.emit_label(&cond_label);
                let current = self.load_variable(loop_var_name, loop_ty)?;
                let end_value = self.lower_type_value(end)?;
                let cmp_name = self.new_temp();
                self.emit_inst(format!(
                    "%{} = lt.{} {}, {}",
                    cmp_name, ty_name, current.repr, end_value.repr
                ));
                self.emit_terminator(format!("br %{}, {}, {}", cmp_name, body_label, end_label));

                self.emit_label(&body_label);
                self.lower_statements(statements)?;

                if !self.terminated {
                    let loop_current = self.load_variable(loop_var_name, loop_ty)?;
                    let step = self.lower_type_value(value)?;
                    let next_name = self.new_temp();
                    self.emit_inst(format!(
                        "%{} = add.{} {}, {}",
                        next_name, ty_name, loop_current.repr, step.repr
                    ));
                    self.emit_inst(format!("store.{} %{}, %{}", ty_name, ptr_name, next_name));
                    self.emit_terminator(format!("jmp {}", cond_label));
                }

                self.emit_label(&end_label);
            }
            AST::Statement(ASTstatement::Import { .. }) => {}
            AST::Statement(ASTstatement::ElseIf { statements, .. })
            | AST::Statement(ASTstatement::Else { statements }) => {
                self.lower_statements(statements)?;
            }
            AST::Statement(other) => {
                return Err(CodegenError::CompilationError(format!(
                    "Unsupported statement for Lamina backend: {:?}",
                    other
                )));
            }
            _ => {
                let _ = self.lower_expr(stmt)?;
            }
        }

        Ok(())
    }

    fn lower_expr(&mut self, expr: &AST) -> Result<LoweredValue, CodegenError> {
        match expr {
            AST::TypeValue(value) => self.lower_type_value(value),
            AST::Logic(logic) => self.lower_logic(logic),
            AST::Statement(ASTstatement::Return { value }) => self.lower_expr(value),
            AST::Statement(ASTstatement::Let {
                value, type_name, ..
            }) => {
                if let Some(v) = value {
                    self.lower_expr(v)
                } else {
                    Ok(LoweredValue {
                        repr: "0".to_string(),
                        ty: type_name.unwrap_or(ASTtypename::I64),
                    })
                }
            }
            AST::Statement(ASTstatement::Assignment { right, .. }) => self.lower_expr(right),
            AST::Statement(other) => Err(CodegenError::CompilationError(format!(
                "Unsupported expression statement for Lamina backend: {:?}",
                other
            ))),
            _ => Err(CodegenError::CompilationError(format!(
                "Unsupported expression for Lamina backend: {:?}",
                expr
            ))),
        }
    }

    fn lower_if_chain(
        &mut self,
        condition: &AST,
        then_statements: &[AST],
        elif: Option<&AST>,
        else_statements: Option<&Vec<AST>>,
        merge_label: &str,
    ) -> Result<(), CodegenError> {
        let then_label = self.new_label("if_then");
        let else_label = self.new_label("if_else");

        let cond = self.lower_expr(condition)?;
        self.emit_terminator(format!("br {}, {}, {}", cond.repr, then_label, else_label));

        self.emit_label(&then_label);
        self.lower_statements(then_statements)?;
        if !self.terminated {
            self.emit_terminator(format!("jmp {}", merge_label));
        }

        self.emit_label(&else_label);
        match elif {
            Some(AST::Statement(ASTstatement::If {
                condition,
                statements,
                elif,
                else_statements,
            })) => {
                self.lower_if_chain(
                    condition,
                    statements,
                    elif.as_deref(),
                    else_statements.as_ref(),
                    merge_label,
                )?;
            }
            Some(other) => {
                return Err(CodegenError::CompilationError(format!(
                    "Invalid elif AST shape for Lamina backend: {:?}",
                    other
                )));
            }
            None => {
                if let Some(body) = else_statements {
                    self.lower_statements(body)?;
                }
            }
        }

        if !self.terminated {
            self.emit_terminator(format!("jmp {}", merge_label));
        }

        Ok(())
    }

    fn lower_logic(&mut self, logic: &astgen::ast::ASTlogic) -> Result<LoweredValue, CodegenError> {
        match logic {
            astgen::ast::ASTlogic::BinaryOperation { left, op, right } => {
                let lhs = self.lower_expr(left)?;
                let rhs = self.lower_expr(right)?;

                let ty = choose_binary_type(lhs.ty, rhs.ty);
                let ty_name = lamina_type(ty)?;
                let result = self.new_temp();

                if let Some(cmp) = lamina_cmp_op(op.clone()) {
                    self.emit_inst(format!(
                        "%{} = {}.{} {}, {}",
                        result, cmp, ty_name, lhs.repr, rhs.repr
                    ));
                    return Ok(LoweredValue {
                        repr: format!("%{}", result),
                        ty: ASTtypename::Bool,
                    });
                }

                self.emit_inst(format!(
                    "%{} = {}.{} {}, {}",
                    result,
                    lamina_binary_op(op.clone())?,
                    ty_name,
                    lhs.repr,
                    rhs.repr
                ));

                Ok(LoweredValue {
                    repr: format!("%{}", result),
                    ty,
                })
            }
        }
    }

    fn lower_type_value(&mut self, value: &ASTtypevalue) -> Result<LoweredValue, CodegenError> {
        match value {
            ASTtypevalue::TypeVoid => Ok(LoweredValue {
                repr: "0".to_string(),
                ty: ASTtypename::TypeVoid,
            }),
            ASTtypevalue::I8(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::I8,
            }),
            ASTtypevalue::I16(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::I16,
            }),
            ASTtypevalue::I32(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::I32,
            }),
            ASTtypevalue::I64(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::I64,
            }),
            ASTtypevalue::U8(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::U8,
            }),
            ASTtypevalue::U16(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::U16,
            }),
            ASTtypevalue::U32(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::U32,
            }),
            ASTtypevalue::U64(v) => Ok(LoweredValue {
                repr: v.to_string(),
                ty: ASTtypename::U64,
            }),
            ASTtypevalue::Bool(v) => Ok(LoweredValue {
                repr: if *v { "true" } else { "false" }.to_string(),
                ty: ASTtypename::Bool,
            }),
            ASTtypevalue::Char(c) => Ok(LoweredValue {
                repr: format!("'{}'", escape_char(*c)),
                ty: ASTtypename::Char,
            }),
            ASTtypevalue::QuotedString(_) => Ok(LoweredValue {
                repr: "0".to_string(),
                ty: ASTtypename::QuotedString,
            }),
            ASTtypevalue::Identifier(name) => {
                let ty = *self.vars.get(name).ok_or_else(|| {
                    CodegenError::VariableNotFound(format!("Unknown variable '{}'", name))
                })?;
                self.load_variable(name, ty)
            }
            ASTtypevalue::FunctionCall { name, args } => {
                let mut lowered_args = Vec::new();
                for arg in args {
                    lowered_args.push(self.lower_expr(arg)?);
                }

                let result_name = self.new_temp();
                let args_text = lowered_args
                    .iter()
                    .map(|arg| arg.repr.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                self.emit_inst(format!("%{} = call @{}({})", result_name, name, args_text));

                let inferred_ty = self
                    .signatures
                    .get(name)
                    .map_or(ASTtypename::I64, |signature| {
                        if signature.params.len() == lowered_args.len() {
                            signature.return_type
                        } else {
                            ASTtypename::I64
                        }
                    });

                Ok(LoweredValue {
                    repr: format!("%{}", result_name),
                    ty: inferred_ty,
                })
            }
            ASTtypevalue::Array(items) => {
                if items.is_empty() {
                    return Err(CodegenError::CompilationError(
                        "Array literals must have at least one element".to_string(),
                    ));
                }

                let element_ty = infer_type_from_typevalue(&items[0]);
                let element_ty_name = lamina_type(element_ty)?;
                let arr_name = self.new_temp();
                self.emit_inst(format!(
                    "%{} = alloc.stack [{} x {}]",
                    arr_name,
                    items.len(),
                    element_ty_name
                ));

                for (index, item) in items.iter().enumerate() {
                    let lowered = self.lower_type_value(item)?;
                    let elem_ptr = self.new_temp();
                    self.emit_inst(format!(
                        "%{} = getelementptr %{}, {}, {}",
                        elem_ptr, arr_name, index, element_ty_name
                    ));
                    self.emit_inst(format!(
                        "store.{} %{}, {}",
                        element_ty_name, elem_ptr, lowered.repr
                    ));
                }

                Ok(LoweredValue {
                    repr: format!("%{}", arr_name),
                    ty: ASTtypename::Array,
                })
            }
        }
    }

    fn load_variable(
        &mut self,
        var_name: &str,
        ty: ASTtypename,
    ) -> Result<LoweredValue, CodegenError> {
        let ptr_name = self.var_ptrs.get(var_name).cloned().ok_or_else(|| {
            CodegenError::VariableNotFound(format!("Unknown variable '{}'", var_name))
        })?;
        let load_name = self.new_temp();
        self.emit_inst(format!(
            "%{} = load.{} %{}",
            load_name,
            lamina_type(ty)?,
            ptr_name
        ));
        Ok(LoweredValue {
            repr: format!("%{}", load_name),
            ty,
        })
    }

    fn emit_default_return(&mut self, return_type: ASTtypename) -> Result<(), CodegenError> {
        match return_type {
            ASTtypename::TypeVoid => self.emit_terminator("ret.void".to_string()),
            _ => self.emit_terminator(format!("ret.{} 0", lamina_type(return_type)?)),
        }
        Ok(())
    }

    fn emit_inst(&mut self, line: String) {
        if !self.terminated {
            self.lines.push(format!("    {}", line));
        }
    }

    fn emit_terminator(&mut self, line: String) {
        if !self.terminated {
            self.lines.push(format!("    {}", line));
            self.terminated = true;
        }
    }

    fn emit_label(&mut self, label: &str) {
        self.lines.push(format!("  {}:", label));
        self.terminated = false;
    }

    fn new_temp(&mut self) -> String {
        let name = format!("tmp{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }
}
