use std::iter::Peekable;
use std::str::Chars;

/// Keel token enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    StrLit(String),
    CharLit(char),
    Comment(String),
    Whitespace(Whitespace),
    SemiColon,
    Colon,
    Equal,
    LowerThan,
    GreaterThan,
    Plus,
    Minus,
    Asterisk,
    Percent,
    Period,
    Comma,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Slash,
    Backslash,
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
                ' ' => self.next(chars, Token::Whitespace(Whitespace::Space)),
                '\n' => self.next(chars, Token::Whitespace(Whitespace::Newline)),
                '\t' => self.next(chars, Token::Whitespace(Whitespace::Tab)),
                '\"' => {
                    let mut s = String::new();
                    chars.next();
                    while let Some(&ch) = chars.peek() {
                        match ch {
                            '\"' => {
                                chars.next();
                                break;
                            }
                            _ => {
                                chars.next();
                                s.push(ch);
                            }
                        }
                    }
                    Ok(Some(Token::StrLit(s)))
                },
                '\\' => self.next(chars, Token::Backslash),
                ';' => self.next(chars, Token::SemiColon),
                '+' => self.next(chars, Token::Plus),
                                '-' => {
                    chars.next();
                    if chars.peek() == Some(&'-') {
                        chars.next();
                        let mut s = String::new();
                        while let Some(&ch) = chars.peek() {
                            match ch {
                                '\n' => {
                                    chars.next();
                                    break;
                                }
                                _ => {
                                    chars.next();
                                    s.push(ch);
                                }
                            }
                        }
                        Ok(Some(Token::Comment(s)))
                    } else {
                        Ok(Some(Token::Minus))
                    }
                },
                '*' => self.next(chars, Token::Asterisk),
                '%' => self.next(chars, Token::Percent),
                '/' => self.next(chars, Token::Slash),
                '.' => self.next(chars, Token::Period),
                ',' => self.next(chars, Token::Comma),
                '(' => self.next(chars, Token::OpenParen),
                ')' => self.next(chars, Token::CloseParen),
                '[' => self.next(chars, Token::OpenBracket),
                ']' => self.next(chars, Token::CloseBracket),
                '{' => self.next(chars, Token::OpenBrace),
                '}' => self.next(chars, Token::CloseBrace),
                '<' => self.next(chars, Token::LowerThan),
                '>' => self.next(chars, Token::GreaterThan),
                '=' => self.next(chars, Token::Equal),
                otherwise => self.next(chars, Token::CharLit(otherwise))
            },
            None => Ok(None),
        }
    }

    fn next(&self, chars: &mut Peekable<Chars>, t: Token) -> Result<Option<Token>, String> {
        chars.next();
        Ok(Some(t))
    }
}