mod logic;
mod statement;
mod types;

use std::fmt;

pub use logic::ASTOperator;
pub use logic::ASTlogic;
pub use statement::ASTstatement;
pub use types::ASTtypecomp;
pub use types::ASTtypename;
pub use types::ASTtypevalue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum AST {
    Statement(ASTstatement),
    TypeName(ASTtypename),
    TypeValue(ASTtypevalue),
    TypeComp(ASTtypecomp),
    Operator(ASTOperator),
    Logic(ASTlogic),
}
impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AST::Statement(statement) => write!(f, "{}", statement),
            AST::TypeName(type_name) => write!(f, "{}", type_name),
            AST::TypeValue(type_value) => write!(f, "{}", type_value),
            AST::TypeComp(type_comp) => write!(f, "{}", type_comp),
            AST::Operator(operator) => write!(f, "{}", operator),
            AST::Logic(logic) => write!(f, "{}", logic),
        }
    }
}
