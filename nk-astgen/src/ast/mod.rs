mod statement;
mod types;
mod logic;

pub use statement::ASTstatement;
pub use types::ASTtypecomp;
pub use types::ASTtypename;
pub use types::ASTtypevalue;
pub use logic::ASTlogic;
pub use logic::ASTOperator;

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
