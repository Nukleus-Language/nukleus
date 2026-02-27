use std::collections::HashMap;

use astgen::ast::{AST, ASTstatement, ASTtypecomp, ASTtypevalue};

use crate::error::CodegenError;

use super::emitter::FunctionEmitter;
use super::helpers::FunctionSignature;

const PRINT_I64_HELPERS: &str = r#"fn @nk_print_i64(i64 %num) -> i64 {
 entry:
  %is_zero = eq.i64 %num, 0
  br %is_zero, nk_print_zero, nk_check_neg

 nk_check_neg:
  %is_negative = lt.i64 %num, 0
  br %is_negative, nk_handle_neg, nk_print_digs

 nk_print_zero:
  %_z = writebyte 48
  ret.i64 0

 nk_handle_neg:
  %_m = writebyte 45
  %abs_num = sub.i64 0, %num
  %_0 = call @nk_print_digits(%abs_num)
  ret.i64 0

 nk_print_digs:
  %_1 = call @nk_print_digits(%num)
  ret.i64 0
}

fn @nk_print_digits(i64 %num) -> i64 {
 entry:
  %is_zero = eq.i64 %num, 0
  br %is_zero, nk_done, nk_cont

 nk_cont:
  %divisor = add.i64 0, 10
  %quotient = div.i64 %num, %divisor
  %temp = mul.i64 %quotient, %divisor
  %remainder = sub.i64 %num, %temp
  %_a = call @nk_print_digits(%quotient)
  %digit = add.i64 %remainder, 48
  %_b = writebyte %digit
  ret.i64 0

 nk_done:
  ret.i64 0
}
"#;

fn ast_uses_integer_print(ast: &[AST]) -> bool {
    for node in ast {
        if let AST::Statement(ASTstatement::Function { statements, .. }) = node
            && statements_use_integer_print(statements)
        {
            return true;
        }
    }
    false
}

fn statements_use_integer_print(statements: &[AST]) -> bool {
    for stmt in statements {
        if ast_uses_integer_print_inner(stmt) {
            return true;
        }
    }
    false
}

fn ast_uses_integer_print_inner(ast: &AST) -> bool {
    match ast {
        AST::Statement(ASTstatement::Print { value, args }) => {
            if needs_integer_print(value) {
                return true;
            }
            for arg in args {
                if needs_integer_print(arg) {
                    return true;
                }
            }
        }
        AST::Statement(ASTstatement::Println { value, args }) => {
            if needs_integer_print(value) {
                return true;
            }
            for arg in args {
                if needs_integer_print(arg) {
                    return true;
                }
            }
        }
        AST::Statement(ASTstatement::If {
            statements: then_s,
            elif,
            else_statements,
            ..
        }) => {
            if statements_use_integer_print(then_s) {
                return true;
            }
            if let Some(elif_ast) = elif
                && ast_uses_integer_print_inner(elif_ast)
            {
                return true;
            }
            if let Some(else_s) = else_statements
                && statements_use_integer_print(else_s)
            {
                return true;
            }
        }
        AST::Statement(ASTstatement::For {
            statements: body, ..
        }) => {
            if statements_use_integer_print(body) {
                return true;
            }
        }
        _ => {}
    }
    false
}

fn needs_integer_print(ast: &AST) -> bool {
    !matches!(ast, AST::TypeValue(ASTtypevalue::QuotedString(_)))
}

#[derive(Debug, Default)]
pub struct LaminaBackend {
    signatures: HashMap<String, FunctionSignature>,
}

impl LaminaBackend {
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
        }
    }

    pub fn compile_ast_to_ir(&mut self, input: &[AST]) -> Result<String, CodegenError> {
        self.collect_signatures(input);

        let mut output = String::new();

        if ast_uses_integer_print(input) {
            output.push_str(PRINT_I64_HELPERS);
            output.push('\n');
        }

        let mut emitted_any = false;
        let mut label_counter = 0usize;

        for node in input {
            if let AST::Statement(ASTstatement::Function {
                name,
                args,
                statements,
                return_type,
                ..
            }) = node
            {
                if emitted_any {
                    output.push('\n');
                }
                emitted_any = true;

                let mut emitter =
                    FunctionEmitter::new(&self.signatures, name, &mut label_counter);
                output.push_str(&emitter.lower_function(name, args, statements, *return_type)?);
            }
        }

        if !emitted_any {
            return Err(CodegenError::CompilationError(
                "No functions found for Lamina backend".to_string(),
            ));
        }

        Ok(output)
    }

    pub fn compile_ast_to_assembly(
        &mut self,
        input: &[AST],
        target: Option<&str>,
    ) -> Result<String, CodegenError> {
        let ir = self.compile_ast_to_ir(input)?;
        self.compile_ir_to_assembly(&ir, target)
    }

    pub fn compile_ir_to_assembly(
        &self,
        ir: &str,
        target: Option<&str>,
    ) -> Result<String, CodegenError> {
        let mut assembly = Vec::new();

        match target {
            Some(target_name) => {
                ::lamina::compile_lamina_ir_to_target_assembly(ir, &mut assembly, target_name)
                    .map_err(|err| CodegenError::CompilationError(err.to_string()))?
            }
            None => ::lamina::compile_lamina_ir_to_assembly(ir, &mut assembly)
                .map_err(|err| CodegenError::CompilationError(err.to_string()))?,
        }

        String::from_utf8(assembly).map_err(|err| CodegenError::CompilationError(err.to_string()))
    }

    fn collect_signatures(&mut self, input: &[AST]) {
        self.signatures.clear();

        for node in input {
            if let AST::Statement(ASTstatement::Function {
                name,
                args,
                return_type,
                ..
            }) = node
            {
                let params = args
                    .iter()
                    .filter_map(|arg| match arg {
                        ASTtypecomp::Argument { type_name, .. } => Some(*type_name),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                self.signatures.insert(
                    name.clone(),
                    FunctionSignature {
                        params,
                        return_type: *return_type,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use astgen::ast::{AST, ASTlogic, ASTstatement, ASTtypecomp, ASTtypename, ASTtypevalue};

    use super::LaminaBackend;

    #[test]
    fn lowers_simple_function_to_valid_lamina_ir() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![AST::Statement(ASTstatement::Return {
                value: Box::new(AST::TypeValue(ASTtypevalue::I64(0))),
            })],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("fn @main() -> i64"));
        assert!(ir.contains("ret.i64 0"));
    }

    #[test]
    fn lowers_params_let_and_assignment() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "add1".to_string(),
            args: vec![ASTtypecomp::Argument {
                type_name: ASTtypename::I64,
                identifier: ASTtypevalue::Identifier("x".to_string()),
            }],
            statements: vec![
                AST::Statement(ASTstatement::Let {
                    name: "a".to_string(),
                    type_name: Some(ASTtypename::I64),
                    value: Some(Box::new(AST::TypeValue(ASTtypevalue::I64(1)))),
                }),
                AST::Statement(ASTstatement::Assignment {
                    left: Box::new(AST::TypeValue(ASTtypevalue::Identifier("a".to_string()))),
                    op: astgen::ast::ASTOperator::AddAssign,
                    right: Box::new(AST::TypeValue(ASTtypevalue::Identifier("x".to_string()))),
                }),
                AST::Statement(ASTstatement::Return {
                    value: Box::new(AST::TypeValue(ASTtypevalue::Identifier("a".to_string()))),
                }),
            ],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("fn @add1(i64 %x) -> i64"));
        assert!(ir.contains("%a_ptr = alloc.stack i64"));
        assert!(ir.contains("store.i64 %a_ptr"));
        assert!(ir.contains("ret.i64"));
    }

    #[test]
    fn lowers_if_expression_shape() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![
                AST::Statement(ASTstatement::Let {
                    name: "a".to_string(),
                    type_name: Some(ASTtypename::I64),
                    value: Some(Box::new(AST::TypeValue(ASTtypevalue::I64(1)))),
                }),
                AST::Statement(ASTstatement::If {
                    condition: Box::new(AST::Logic(ASTlogic::BinaryOperation {
                        left: Box::new(AST::TypeValue(ASTtypevalue::Identifier("a".to_string()))),
                        op: astgen::ast::ASTOperator::Greater,
                        right: Box::new(AST::TypeValue(ASTtypevalue::I64(0))),
                    })),
                    statements: vec![AST::Statement(ASTstatement::Print {
                        value: Box::new(AST::TypeValue(ASTtypevalue::QuotedString("ok".into()))),
                        args: vec![],
                    })],
                    elif: None,
                    else_statements: Some(vec![AST::Statement(ASTstatement::Print {
                        value: Box::new(AST::TypeValue(ASTtypevalue::QuotedString("no".into()))),
                        args: vec![],
                    })]),
                }),
                AST::Statement(ASTstatement::Return {
                    value: Box::new(AST::TypeValue(ASTtypevalue::I64(0))),
                }),
            ],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("br %"));
        assert!(ir.contains("if_then_"));
        assert!(ir.contains("if_else_"));
        assert!(ir.contains("if_merge_"));
    }

    #[test]
    fn lowers_print_string_to_writebyte_loop() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![AST::Statement(ASTstatement::Print {
                value: Box::new(AST::TypeValue(ASTtypevalue::QuotedString("hi".into()))),
                args: vec![],
            })],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("writebyte"));
        assert!(ir.contains("add.i64 104")); // 'h'
        assert!(ir.contains("add.i64 105")); // 'i'
    }

    #[test]
    fn lowers_format_string_with_args() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![
                AST::Statement(ASTstatement::Let {
                    name: "x".to_string(),
                    type_name: Some(ASTtypename::I64),
                    value: Some(Box::new(AST::TypeValue(ASTtypevalue::I64(42)))),
                }),
                AST::Statement(ASTstatement::Println {
                    value: Box::new(AST::TypeValue(ASTtypevalue::QuotedString("x={}".into()))),
                    args: vec![AST::TypeValue(ASTtypevalue::Identifier("x".to_string()))],
                }),
            ],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("writebyte"));
        assert!(ir.contains("call @nk_print_i64"));
        assert!(!ir.contains("print %"));
    }

    #[test]
    fn lowers_println_integer_via_writebyte_helper() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![AST::Statement(ASTstatement::Println {
                value: Box::new(AST::TypeValue(ASTtypevalue::I64(42))),
                args: vec![],
            })],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("call @nk_print_i64"));
        assert!(ir.contains("writebyte 10"));
        assert!(!ir.contains("print %"));
    }

    #[test]
    fn lowers_println_string_to_writebyte_and_newline() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![AST::Statement(ASTstatement::Println {
                value: Box::new(AST::TypeValue(ASTtypevalue::QuotedString("x".into()))),
                args: vec![],
            })],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let ir = backend.compile_ast_to_ir(&input).expect("must lower");

        assert!(ir.contains("writebyte 10"));
    }

    #[test]
    fn compiles_to_assembly_smoke() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![],
            statements: vec![AST::Statement(ASTstatement::Return {
                value: Box::new(AST::TypeValue(ASTtypevalue::I64(0))),
            })],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();
        let asm = backend
            .compile_ast_to_assembly(&input, None)
            .expect("assembly generation should work");

        assert!(!asm.trim().is_empty());
    }

    #[test]
    fn benchmark_backend_pipeline() {
        let input = vec![AST::Statement(ASTstatement::Function {
            public: true,
            name: "main".to_string(),
            args: vec![ASTtypecomp::Argument {
                type_name: ASTtypename::I64,
                identifier: ASTtypevalue::Identifier("x".to_string()),
            }],
            statements: vec![
                AST::Statement(ASTstatement::Let {
                    name: "sum".to_string(),
                    type_name: Some(ASTtypename::I64),
                    value: Some(Box::new(AST::TypeValue(ASTtypevalue::I64(0)))),
                }),
                AST::Statement(ASTstatement::For {
                    start: ASTtypevalue::Identifier("sum".to_string()),
                    end: ASTtypevalue::I64(100),
                    value: ASTtypevalue::I64(1),
                    statements: vec![AST::Statement(ASTstatement::Assignment {
                        left: Box::new(AST::TypeValue(ASTtypevalue::Identifier("sum".to_string()))),
                        op: astgen::ast::ASTOperator::AddAssign,
                        right: Box::new(AST::TypeValue(ASTtypevalue::Identifier("x".to_string()))),
                    })],
                }),
                AST::Statement(ASTstatement::Return {
                    value: Box::new(AST::TypeValue(ASTtypevalue::Identifier("sum".to_string()))),
                }),
            ],
            return_type: ASTtypename::I64,
        })];

        let mut backend = LaminaBackend::new();

        let ir_start = Instant::now();
        for _ in 0..200 {
            let _ = backend
                .compile_ast_to_ir(&input)
                .expect("ir lowering must work");
        }
        let ir_elapsed = ir_start.elapsed();

        let asm_start = Instant::now();
        for _ in 0..10 {
            let _ = backend
                .compile_ast_to_assembly(&input, None)
                .expect("assembly lowering must work");
        }
        let asm_elapsed = asm_start.elapsed();

        let _ = (ir_elapsed, asm_elapsed);
    }
}
