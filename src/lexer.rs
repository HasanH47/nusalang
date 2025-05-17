// src/lexer.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Func,
    Let,
    Print,
    Ident(String),
    Number(f64),
    String(String),
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
}

#[derive(Debug)]
pub enum LexError {
    InvalidChar(char),
    InvalidNumber(String),
    UnterminatedString,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidChar(c) => write!(f, "Invalid character: '{}'", c),
            LexError::InvalidNumber(s) => write!(f, "Invalid number format: '{}'", s),
            LexError::UnterminatedString => write!(f, "Unterminated string literal"),
        }
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RBrace);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match number.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err(LexError::InvalidNumber(number)),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "func" => tokens.push(Token::Func),
                    "let" => tokens.push(Token::Let),
                    "print" => tokens.push(Token::Print),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            '\'' => {
                chars.next();
                let mut string_value = String::new();

                while let Some(&c) = chars.peek() {
                    if c == '\'' {
                        chars.next();
                        break;
                    } else if c == '\\' {
                        chars.next();
                        if let Some(next) = chars.next() {
                            match next {
                                'n' => string_value.push('\n'),
                                't' => string_value.push('\t'),
                                'r' => string_value.push('\r'),
                                '\\' => string_value.push('\\'),
                                '\'' => string_value.push('\''),
                                '"' => string_value.push('"'),
                                _ => string_value.push(next),
                            }
                        } else {
                            return Err(LexError::UnterminatedString);
                        }
                    } else {
                        string_value.push(c);
                        chars.next();
                    }
                }

                tokens.push(Token::String(string_value));
            }

            '"' => {
                chars.next();
                let mut string_value = String::new();

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        break;
                    } else if c == '\\' {
                        chars.next();
                        if let Some(next) = chars.next() {
                            match next {
                                'n' => string_value.push('\n'),
                                't' => string_value.push('\t'),
                                'r' => string_value.push('\r'),
                                '\\' => string_value.push('\\'),
                                '\'' => string_value.push('\''),
                                '"' => string_value.push('"'),
                                _ => string_value.push(next),
                            }
                        } else {
                            return Err(LexError::UnterminatedString);
                        }
                    } else {
                        string_value.push(c);
                        chars.next();
                    }
                }

                tokens.push(Token::String(string_value));
            }
            _ => return Err(LexError::InvalidChar(ch)),
        }
    }

    Ok(tokens)
}
