use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
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
    #[token = ";"] SemiColon,
    #[token = "="] Assign,
    #[token = ">"] GreaterThan,
    #[token = "<"] LowerThan,
    #[token = ","] Comma,
    #[token = "."] Period,
    
    #[token = ";"]
    #[token = "\n"]
    Separator,

    #[regex = "[a-zA-Z_$][a-zA-Z0-9_$]*"] Identifier,
    #[regex = "[0-9]+"]                   IntegerLit,

    #[regex = "\"([^\"\\\\]|\\\\.)*\""] StringLit,
    #[regex = "'([^'\\\\]|\\\\.)'"]     CharLit,
    #[regex = "--([^\n]|.)*"]           Comment,

    #[token = "let"] KeywordLet,
    #[token = "data"] KeywordData,
    #[token = "match"] KeywordMatch,
    #[token = "with"] KeywordWith,
    #[regex = "in|case|cocase|of|assert|rewrite|rule|do|->|<-|=>"]
    ReservedKeyword,
}
