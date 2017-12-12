use std::iter::Peekable;
use std::str::Chars;
use lexer::tokens::Token;

pub struct Lexer<'a> {
    last: Option<&'a Token>,
    stream: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Lexer { stream: s.chars().peekable(), last: None }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, String> {
        let mut list = Vec::new();

        while let Some(c) = self.stream.next() {
            match c {
                _x if _x.is_whitespace() => (),
                '(' => list.push(Token::LeftParen),
                ')' => list.push(Token::RightParen),
                '[' => list.push(Token::LeftBracket),
                ']' => list.push(Token::RightBracket),
                '{' => list.push(Token::LeftBrace),
                '}' => list.push(Token::RightBrace),
                '+' => list.push(Token::Plus),
                '-' => list.push(Token::Minus),
                '*' => list.push(Token::Asterisk),
                '/' => list.push(Token::Slash),
                '=' => list.push(Token::Assign), // TODO EQUALITY
                'A'...'Z' | 'a'...'z' | '_' => list.push(self.ident(c)),
                _ => return Err(String::from("Unknown error"))
            }
        }

        Ok(list)
    }

    fn ident(&mut self, c: char) -> Token {
        let mut result = String::new();
        result.push(c);

        while let Some(&c) = self.stream.peek() {
            match c {
                x if x.is_alphanumeric() || x == '_' => {
                    result.push(x);
                    self.stream.next();
                }
                _ => break,
            }
        }

        match result.as_ref() {
            "true" => return Token::Boolean(true),
            "false" => return Token::Boolean(false),
            "let" => return Token::Let,
            x => return Token::Identifier(x.to_string()),
        }
    }
}