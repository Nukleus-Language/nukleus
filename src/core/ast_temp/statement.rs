#[derive(Clone, Debug)]
pub enum Statement {
    
    Function {
        name: String,
        parameters: Vec<String>,
        return_type: Option<Type>,
        body: Box<AST>,
    },
    Error
}
