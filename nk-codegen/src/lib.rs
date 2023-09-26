use astgen::ast::*;
use astgen::AST;
pub fn generate_ir(asts: Vec<AST>) -> String {
    let mut ir = String::new();
    for ast in asts {
        ir += &generate_ast_ir(ast);
    }
    ir
}

fn generate_ast_ir(ast: AST) -> String {
    match ast {
        /*AST::Statement(statement) => match statement {
            ASTstatement::Function { public, name, args, statements, return_type } => {
                let args_string = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let statements_string = statements
                    .iter()
                    .map(|statement| statement.to_string())
                    .collect::<Vec<String>>()
                    .join("\n");
                format!(
                    "function {}({}) {{\n{}\n}}",
                    name, args_string, statements_string
                )
            },
            ASTstatement::Print { value } => format!("print {};\n", value),
            ASTstatement::Println { value } => format!("println {};\n", value),
            ASTstatement::For { start, end, value, statements } => {
                let mut ir = format!("for {} to {} step {} {{\n", start, end, value);
                for stmt in statements {
                    ir += &generate_ast_ir(stmt);
                }
                ir += "}\n";
                ir
            },
            ASTstatement::If { condition, statements } => {
                let mut ir = format!("if {} {{\n", condition.iter().map(|cond| cond.to_string()).collect::<Vec<_>>().join(" "));
                for stmt in statements {
                    ir += &generate_ast_ir(stmt);
                }
                ir += "}\n";
                ir
            },
            ASTstatement::Return { value } => format!("return {};\n", value),
            ASTstatement::RemAssign { l_var, r_var } => format!("{} = {};\n", l_var, r_var),
            ASTstatement::Import { name } => format!("import {};\n", name),
            ASTstatement::ElseIf { condition, statements } => {
                let mut ir = format!("else if {} {{\n", condition.iter().map(|cond| cond.to_string()).collect::<Vec<_>>().join(" "));
                for stmt in statements {
                    ir += &generate_ast_ir(stmt);
                }
                ir += "}\n";
                ir
            }
            ASTstatement::Else { statements } => {
                let mut ir = String::new();
                for stmt in statements {
                    ir += &generate_ast_ir(stmt);
                }
                ir
            }
            _ => "".to_string(),
        },
        AST::TypeValue(value) => match value {
            ASTtypevalue::TypeVoid => "void".to_string(),
            ASTtypevalue::U8(num) => num.to_string(),
            ASTtypevalue::I8(num) => num.to_string(),
            ASTtypevalue::U16(num) => num.to_string(),
            ASTtypevalue::I16(num) => num.to_string(),
            ASTtypevalue::U32(num) => num.to_string(),
            ASTtypevalue::I32(num) => num.to_string(),
            // ASTtypevalue::F32(num) => num.to_string(),
            // ASTtypevalue::F64(num) => num.to_string(),
            ASTtypevalue::U64(num) => num.to_string(),
            ASTtypevalue::I64(num) => num.to_string(),
            ASTtypevalue::Identifier(ident) => ident,
            ASTtypevalue::QuotedString(s) => format!("\"{}\"", s),
            ASTtypevalue::Bool(b) => b.to_string(),
        },
        AST::Logic(logic) => match logic {
            ASTlogic::BinaryOperation { left, op, right } => format!("{} {} {}", generate_ast_ir(*left), op, generate_ast_ir(*right)),
            // ... handle other logic types
        },
        AST::TypeComp(comp) => match comp {
            ASTtypecomp::Array(val) => format!("[{}]", val.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
            ASTtypecomp::Argument { type_name, identifier } => format!("{} {}", type_name, identifier),
            ASTtypecomp::FunctionCall { name, args } => format!("{}({})", name, args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ")),
        },*/
        AST::Statement(statement) => statement.to_string(),
        AST::TypeName(type_name) => type_name.to_string(),
        AST::TypeValue(type_value) => type_value.to_string(),
        AST::TypeComp(type_comp) => type_comp.to_string(),
        AST::Operator(operator) => operator.to_string(),
        AST::Logic(logic) => logic.to_string(),
    }
}

// Example usage:
// let ir_code = generate_ir(your_ast_vector);
