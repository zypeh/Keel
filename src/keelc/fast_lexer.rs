use logos::Logos;
// use logos_derive::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[end] EndOfFile,
    #[error] TokenizerError,
    #[token = "("] OpenParen,
    #[token = ")"] CloseParen,
    #[token = "["] OpenBracket,
    #[token = "]"] CloseBracket,
    #[token = "{"] OpenBrace,
    #[token = "}"] CloseBrace,
    #[token = "+"] Plus,
    #[token = "-"] Minus,
    #[token = "*"] Asterisk,
    #[token = "/"] Slash,
    #[token = "\\"] BackSlash,
    #[token = ":"] Colon,
    #[token = "::"] DoubleColon,
    #[token = ";"] SemiColon,
    #[token = "="] Equal,
    #[token = "=>"] FatArrow,
    #[token = "->"] ArrowRight,
    #[token = "<-"] ArrowLeft,
    
    #[token = "let"] OpDeclaration,
    #[token = "if"] OpIf,
    #[token = "else"] OpElse,
    #[token = "then"] OpThen,
    
    #[regex = "[a-zA-Z_$][a-zA-Z0-9_$]*"]
    Identifier,

    #[regex = "\"([^\"\\\\]|\\\\.)*\""]
    StringLit,

    #[regex = "--[^\n]*"]
    Comment,
}
