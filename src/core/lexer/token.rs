#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Tokens
{
    Integer(usize),
    Decimal(f64),
    Identifier(String),
    QuotedString(String),
    Asterisk,
    At, 
    Carat, 
    CloseParen, 
    CloseSquare, 
    Colon,
    Dot, 
    End,
    Equals,
    Minus, 
    OpenParen, 
    OpenSquare, 
    Plus,
    Semicolon,
    Slash,

}
