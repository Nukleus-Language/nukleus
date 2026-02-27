use std::collections::HashMap;

use astgen::ast::{ASTstatement, ASTtypecomp, AST};

use crate::error::CodegenError;

use super::emitter::FunctionEmitter;
use super::helpers::FunctionSignature;

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
        let mut emitted_any = false;

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

                let mut emitter = FunctionEmitter::new(&self.signatures);
                output.push_str(&emitter.lower_function(name, args, statements, *return_type)?);
            }
        }

        if output.is_empty() {
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
