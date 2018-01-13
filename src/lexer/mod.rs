pub mod tokens;

use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;
use lexer::tokens::{Token, TokenType};

pub struct Tokenizer<'a> {
    stream: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer {
            stream: s.chars().peekable(),
        }
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            string: None,
        }
    }

    pub fn make_token_str(&self, token_type: TokenType, string: String) -> Token {
        Token {
            token_type,
            string: Some(string),
        }
    }

    pub fn parse(&mut self) -> Result<VecDeque<Token>, String> {
        let mut list = VecDeque::new();
        self.parse_into_vec(&mut list)?;
        Ok(list)
    }

    pub fn parse_into_vec(&mut self, list: &mut VecDeque<Token>) -> Result<(), String> {
        while let Some(c) = self.stream.next() {
            self.parse_char_into_vec(c, list)?;
        }
        Ok(())
    }

    pub fn parse_char_into_vec(
        &mut self,
        c: char,
        list: &mut VecDeque<Token>,
    ) -> Result<(), String> {
        match c {
            '\n' => list.push_back(self.make_token(TokenType::NewLine)),
            _x if _x.is_whitespace() => (),
            ';' => list.push_back(self.make_token(TokenType::Semi)),
            '(' => list.push_back(self.make_token(TokenType::LeftParen)),
            ')' => list.push_back(self.make_token(TokenType::RightParen)),
            '[' => list.push_back(self.make_token(TokenType::LeftBracket)),
            ']' => list.push_back(self.make_token(TokenType::RightBracket)),
            '{' => list.push_back(self.make_token(TokenType::LeftBrace)),
            '}' => list.push_back(self.make_token(TokenType::RightBrace)),
            ':' => list.push_back(self.make_token(TokenType::Colon)),
            '+' => list.push_back(self.make_token(TokenType::Plus)),
            '-' => list.push_back(self.make_token(TokenType::Minus)),
            '*' => list.push_back(self.make_token(TokenType::Asterisk)),
            '/' => list.push_back(self.make_token(TokenType::Slash)),
            '.' => list.push_back(self.make_token(TokenType::Dot)),
            ',' => list.push_back(self.make_token(TokenType::Comma)),
            '=' => match self.stream.peek() {
                Some(&'=') => {
                    list.push_back(self.make_token(TokenType::Eq));
                    self.stream.next();
                }
                _ => list.push_back(self.make_token(TokenType::Assign)),
            },
            '<' => match self.stream.peek() {
                Some(&'=') => {
                    list.push_back(self.make_token(TokenType::Lte));
                    self.stream.next();
                }
                _ => list.push_back(self.make_token(TokenType::Lt)),
            },
            '>' => match self.stream.peek() {
                Some(&'=') => {
                    list.push_back(self.make_token(TokenType::Gt));
                    self.stream.next();
                }
                _ => list.push_back(self.make_token(TokenType::Gte)),
            },
            '"' => self.string('"', true, list)?,
            '\'' => self.string('\'', false, list)?,
            'A'...'Z' | 'a'...'z' | '_' => list.push_back(self.name(c)),
            '0'...'9' => list.push_back(self.number(c)?),
            _ => return Err(format!("Illegal character {}", c)),
        }
        Ok(())
    }

    fn string(
        &mut self,
        delim: char,
        template: bool,
        list: &mut VecDeque<Token>,
    ) -> Result<(), String> {
        let mut buffer = String::new();
        let mut not_terminated = true;

        while let Some(&c) = self.stream.peek() {
            match c {
                '$' if template => {
                    self.stream.next();

                    if let Some(&'{') = self.stream.peek() {
                        self.stream.next();

                        list.push_back(self.make_token_str(TokenType::String, buffer.to_owned()));
                        list.push_back(self.make_token(TokenType::Plus));
                        buffer.clear();

                        list.push_back(self.make_token(TokenType::LeftParen));

                        let mut braces = 0;
                        while let Some(c) = self.stream.next() {
                            match c {
                                '}' => if braces == 0 {
                                    break;
                                } else {
                                    braces -= 1;
                                    self.parse_char_into_vec(c, list)?;
                                },
                                '{' => {
                                    braces += 1;
                                    self.parse_char_into_vec(c, list)?;
                                }
                                _ => self.parse_char_into_vec(c, list)?,
                            }
                        }
                        list.push_back(self.make_token(TokenType::RightParen));
                    } else if self.stream.peek().map_or(false, |c| c.is_alphabetic()) {
                        list.push_back(self.make_token_str(TokenType::String, buffer.to_owned()));
                        list.push_back(self.make_token(TokenType::Plus));
                        buffer.clear();

                        buffer.push(self.stream.next().unwrap());

                        while self.stream.peek().map_or(false, |c| c.is_alphabetic()) {
                            buffer.push(self.stream.next().unwrap());
                        }
                    } else {
                        buffer.push(c);
                        continue;
                    }

                    list.append(&mut Tokenizer::new(buffer.as_ref()).parse()?);
                    list.push_back(self.make_token(TokenType::Plus));
                    buffer.clear();
                }
                x if x == delim => {
                    self.stream.next();
                    not_terminated = false;
                    break;
                }
                _ => {
                    buffer.push(c);
                    self.stream.next();
                }
            }
        }

        if not_terminated {
            return Err(String::from("Unterminated"));
        }

        list.push_back(self.make_token_str(TokenType::String, buffer));

        Ok(())
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
            "true" => self.make_token(TokenType::True),
            "false" => self.make_token(TokenType::False),
            "let" => self.make_token(TokenType::Let),
            "null" => self.make_token(TokenType::Null),
            x => self.make_token_str(TokenType::Identifier, x.to_string()),
        }
    }

    fn number(&mut self, c: char) -> Result<Token, String> {
        let mut buffer = String::new();

        if c == '0' {
            match self.stream.next() {
                Some('x') => {
                    while let Some(&c) = self.stream.peek() {
                        match c {
                            '1'...'9' | 'A'...'F' => {
                                buffer.push(c);
                                self.stream.next();
                            }
                            _ => break,
                        }
                    }

                    use std::i32;
                    return Ok(
                        self.make_token_str(
                            TokenType::Int,
                            i32::from_str_radix(&buffer, 16)
                                .map_err(|_| "Can't parse hex num")?
                                .to_string(),
                        ),
                    );
                }
                Some('b') => {
                    while let Some(&c) = self.stream.peek() {
                        match c {
                            '0'...'1' => {
                                buffer.push(c);
                                self.stream.next();
                            }
                            _ => break,
                        }
                    }

                    return Ok(
                        self.make_token_str(
                            TokenType::Int,
                            i32::from_str_radix(&buffer, 2)
                                .map_err(|_| "Can't parse binary num")?
                                .to_string(),
                        ),
                    );
                }
                Some(n) => buffer.push(n),
                _ => {}
            }
        } else {
            buffer.push(c);
        }

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

    fn num_buffer_proc(&mut self, buffer: &mut String, is_int: bool) -> Result<Token, String> {
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

            return Ok(
                self.make_token_str(
                    TokenType::Number,
                    buffer
                        .parse::<f64>()
                        .map_err(|_| "Can not parse number")?
                        .to_string(),
                ),
            );
        }

        if is_int {
            Ok(
                self.make_token_str(
                    TokenType::Int,
                    buffer
                        .parse::<i32>()
                        .map_err(|_| "Can not parse int")?
                        .to_string(),
                ),
            )
        } else {
            Ok(
                self.make_token_str(
                    TokenType::Number,
                    buffer
                        .parse::<f64>()
                        .map_err(|_| "Can not parse number")?
                        .to_string(),
                ),
            )
        }
    }
}
