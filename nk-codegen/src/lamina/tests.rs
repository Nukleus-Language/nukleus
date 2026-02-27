use std::time::Instant;

use astgen::ast::{ASTlogic, ASTstatement, ASTtypecomp, ASTtypename, ASTtypevalue, AST};

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

    println!(
        "lamina backend bench: ir_200={:?}, asm_10={:?}",
        ir_elapsed, asm_elapsed
    );
}
