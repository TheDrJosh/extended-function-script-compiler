
pub struct Token<'a> {
    pub start: usize,
    pub value: &'a mut str,
    pub type1: TokenType,
}

pub enum TokenType {
    Identifier,
    Number,

    LParen, // (
    RParen, // )
    LBracket, // [
    RBracket, // ]
    LBraces, // {
    RBraces, // }

    Plus, // +
    Minus, // -
    Multi, // *
    Div, // /

    Equal, // ==
    NotEqual, // !=
    Less, // <
    LessOrEqual, // <=
    Greater, // >
    GreaterOrEqual, // >=

    
}