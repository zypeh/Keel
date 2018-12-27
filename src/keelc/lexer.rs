use std::iter::Peekable;
use std::str::Chars;

/// Keel token enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    StrLit(String),
    CharLit(char),
    Comma,
    Whitespace(Whitespace),
    SemiColon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Whitespace {
    Space,
    Tab,
    Newline,
}

pub struct Lexer {
    pub input: String,
    pub line: u64,
    pub col: u64,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            line: 1,
            col: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, String> {
        let mut peekable = self.input.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.next_token(&mut peekable)? {
            match &token {
                Token::Whitespace(Whitespace::Newline) => {
                    self.line += 1;
                    self.col = 1;
                }
                _ => self.col += 1,
            }
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next_token(&self, chars: &mut Peekable<Chars>) -> Result<Option<Token>, String> {
        match chars.peek() {
            Some(&ch) => match ch {
                ' ' => {
                    chars.next();
                    Ok(Some(Token::Whitespace(Whitespace::Space)))
                }
                '\t' => {
                    chars.next();
                    Ok(Some(Token::Whitespace(Whitespace::Tab)))
                }
                '\n' => {
                    chars.next();
                    Ok(Some(Token::Whitespace(Whitespace::Newline)))
                }
            }
        }
    }
}