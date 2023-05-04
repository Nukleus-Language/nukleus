mod statement;
mod types;

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
}
