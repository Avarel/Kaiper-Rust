#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub string: Option<String>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,

    Int,
    Number,
    True,
    False,
    String,
    Identifier,
    Null,

    Let,
    Assign,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Backslash,
    Caret,
    Percent,

    Eq,
    Gte,
    Gt,
    Lt,
    Lte,

    Dot,
    Comma,
    Colon,

    NewLine,
    Semi,
}
