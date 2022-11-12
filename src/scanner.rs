use std::num::ParseFloatError;

use crate::token::{try_to_keyword, Literal, Token, TokenType};

#[derive(Debug, Clone)]
pub enum ScannerError {
    CharAt,
    TokenScan(String),
    FloatParse(ParseFloatError),
    IdentifierParse(String),
}

impl From<ParseFloatError> for ScannerError {
    fn from(value: ParseFloatError) -> Self {
        ScannerError::FloatParse(value)
    }
}

#[derive(Debug, Clone)]
pub struct Scanner {
    pub source:  String,
    pub tokens:  Vec<Token>,
    pub start:   usize,
    pub current: usize,
    pub line:    usize,
}

type ScannerResult<T> = Result<T, ScannerError>;

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source:  source.to_string(),
            tokens:  vec![],
            start:   0,
            current: 0,
            line:    1,
        }
    }

    pub fn try_scan_tokens(&mut self) -> ScannerResult<Vec<Token>> {
        while self.is_at_end() {
            self.start = self.current;
            self.try_scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            self.line,
        ));

        Ok(vec![])
    }

    fn try_scan_token(&mut self) -> ScannerResult<()> {
        let c = self.try_advance()?;

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => self.add_token(
                if self.clone().is_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                },
                None,
            ),
            '=' => self.add_token(
                if self.clone().is_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                },
                None,
            ),
            '<' => self.add_token(
                if self.clone().is_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                },
                None,
            ),
            '>' => self.add_token(
                if self.clone().is_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                },
                None,
            ),
            '/' => {
                if self.is_at_end() {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.try_advance().unwrap();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            '\n' => {
                self.line += 1;
            }
            'o' => {
                if self.peek()? == 'r' {
                    self.add_token(TokenType::Or, None)
                }
            }
            // Silently ignore
            ' ' | '\r' | '\t' => {}
            c => {
                if c.is_numeric() {
                    self.number()?;
                } else if c.is_alphabetic() {
                    self.identifier()?;
                } else {
                    Err(ScannerError::TokenScan(c.to_string()))?
                }
            }
        };

        Ok(())
    }

    fn is_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.char_at(self.current)
                .map(|c| {
                    if c == expected {
                        self.current += 1;
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
        }
    }

    fn peek(&self) -> ScannerResult<char> {
        if self.is_at_end() {
            Ok('\0')
        } else {
            self.char_at(self.current)
        }
    }

    fn peek_next(&self) -> ScannerResult<char> {
        if self.current + 1 >= self.source.len() {
            Ok('\0')
        } else {
            self.char_at(self.current + 1)
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            literal,
            self.line,
        ))
    }

    fn try_advance(&mut self) -> ScannerResult<char> {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn char_at(&self, index: usize) -> ScannerResult<char> {
        self.source.chars().nth(index).ok_or(ScannerError::CharAt)
    }

    fn identifier(&self) -> ScannerResult<()> {
        while self.peek()?.is_alphanumeric() {
            self.clone().try_advance()?;
        }
        let text = &self.source[self.start..self.current];
        let token_type = try_to_keyword(text).unwrap_or(TokenType::Identifier);
        self.clone().add_token(token_type, None);

        Ok(())
    }

    fn number(&self) -> ScannerResult<()> {
        while self.peek()?.is_numeric() {
            self.clone().try_advance()?;
        }

        if self.peek()? == '.' && self.peek_next()?.is_numeric() {
            self.clone().try_advance()?;

            while self.peek()?.is_numeric() {
                self.clone().try_advance()?;
            }
        }

        let parsed_float = self.source[self.start..self.current]
            .parse::<f64>()
            .map_err(ScannerError::from)?;
        self.clone()
            .add_token(TokenType::Number, Some(Literal::Float(parsed_float)));

        Ok(())
    }
}
