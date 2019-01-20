use logos::Logos;

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
    #[token = "|"] Pipeline,
    #[token = "::"] DoubleColon,
    #[token = ";"] SemiColon,
    #[token = "="] Assign,
    #[token = ">"] GreaterThan,
    #[token = "<"] LowerThan,
    #[token = "=>"] FatArrow,
    #[token = ">="] GreaterThanEqual,
    #[token = "<="] LowerThanEqual,
    #[token = "->"] ArrowRight,
    #[token = "<-"] ArrowLeft,
    
    #[token = ";"]
    #[token = "\n"]
    Separator,

    #[token = "let"] OpDeclaration,
    #[token = "if"] OpIf,
    #[token = "else"] OpElse,
    #[token = "then"] OpThen,
    #[token = "match"] OpMatch,

    #[regex = "[a-zA-Z_$][a-zA-Z0-9_$]*"]
    Identifier,

    #[regex = "\"([^\"\\\\]|\\\\.)*\""]
    StringLit,

    #[regex = "--([^\n]|.)*"]
    Comment,
}
