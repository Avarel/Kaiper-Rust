use std::iter::Peekable;
use std::str::Chars;
use lexer::tokens::Token;

pub struct Tokenizer<'a> {
    last: Option<&'a Token>,
    stream: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer {
            stream: s.chars().peekable(),
            last: None,
        }
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
                '.' => list.push(Token::Dot),
                '=' => match self.stream.peek() {
                    Some(&'=') => {
                        list.push(Token::Eq);
                        self.stream.next();
                    }
                    _ => list.push(Token::Assign),
                }
                '<' => match self.stream.peek() {
                    Some(&'=') => {
                        list.push(Token::Lte);
                        self.stream.next();
                    }
                    _ => list.push(Token::Lt),
                }
                '>' => match self.stream.peek() {
                    Some(&'=') => {
                        list.push(Token::Gt);
                        self.stream.next();
                    }
                    _ => list.push(Token::Gte),
                }
                'A'...'Z' | 'a'...'z' | '_' => list.push(self.name(c)),
                '0'...'9' => list.push(self.number(c)?),
                _ => return Err(format!("Illegal character {}", c)),
            }
        }

        Ok(list)
    }

    fn name(&mut self, c: char) -> Token {
        let mut buffer = String::new();
        buffer.push(c);

        while let Some(&c) = self.stream.peek() {
            match c {
                x if x.is_alphanumeric() || x == '_' => {
                    buffer.push(x);
                    self.stream.next();
                }
                _ => break,
            }
        }

        match buffer.as_ref() {
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "let" => Token::Let,
            "null" => Token::Null,
            x => Token::Identifier(x.to_string()),
        }
    }

    fn number(&mut self, c: char) -> Result<Token, String> {
        let mut buffer = String::new();

        // if c == '0' {
        //     match self.stream.next() {
        //         Some('x') => {
                    
        //         }
        //         Some('b') => {

        //         }
        //         _ => {
        //             self.num_buffer_fill(&mut buffer);
        //         }
        //     }
        // }

        buffer.push(c);

        while let Some(&c) = self.stream.peek() {
            match c {
                '0'...'9' => {
                    buffer.push(c);
                    self.stream.next();
                }
                '.' => {
                    buffer.push(c);
                    self.stream.next();

                    self.num_buffer_fill(&mut buffer);
                    return self.num_buffer_proc(&mut buffer, false);
                }
                '_' => {
                    self.stream.next();
                }
                _ => break,
            }
        }

        self.num_buffer_proc(&mut buffer, true)
    }

    fn num_buffer_fill(&mut self, buffer: &mut String) {
        while let Some(&c) = self.stream.peek() {
            match c {
                '0'...'9' => {
                    buffer.push(c);
                    self.stream.next();
                }
                '_' => {
                    self.stream.next();
                }
                _ => break,
            }
        }
    }

    fn num_buffer_proc(&mut self, buffer: &mut String, isInt: bool) -> Result<Token, String> {
        if let Some(&'e') = self.stream.peek() {
            buffer.push('e');
            self.stream.next();

            if let Some(&'+') = self.stream.peek() {
                buffer.push('+');
                self.stream.next();
            } else if let Some(&'-') = self.stream.peek() {
                buffer.push('-');
                self.stream.next();
            }

            self.num_buffer_fill(buffer);

            return Ok(Token::Number(
                buffer.parse::<f64>().map_err(|_| "Can not parse number")?,
            ));
        }

        if isInt {
            Ok(Token::Int(
                buffer.parse::<>().map_err(|_| "Can not parse int")?,
            ))
        } else {
            Ok(Token::Number(
                buffer.parse::<>().map_err(|_| "Can not parse number")?,
            ))
        }
    }
}