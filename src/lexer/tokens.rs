#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,

    Int(i32),
    Number(f64),
    Boolean(bool),
    String(String),
    Identifier(String),

    Let,
    Assign,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Backslash,
    Caret,
    Percent,

    

    Dot,
}