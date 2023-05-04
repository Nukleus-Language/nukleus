mod statement;
mod types;

pub use types::ASTtypename;
pub use types::ASTtypevalue;
pub use types::ASTtypecomp;
pub use statement::ASTstatement;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[allow(dead_code)]
pub enum AST{
    Statement(ASTstatement),
    TypeName(ASTtypename),
    TypeValue(ASTtypevalue),
    TypeComp(ASTtypecomp),
}
