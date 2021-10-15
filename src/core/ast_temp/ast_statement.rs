#[derive(Clone, Debug)]
pub enum Statement {
    Import {
        path: String,
        name: String,
    },
    Class {
        public: bool,
        name: String,
        attributes: Vec<(String, String)>
    },
    Function {
        public: bool,
        name: String,
        parameters: Option<Vec<function::Parameter>>,
        returns: Option<Vec<String>>,
        body: Vec<expression::Expression>
    },
    Error
}
