extern crate logos;

mod token;

pub use self::token::Token;
pub use logos::{Logos, lookup};
pub type Lexer<S> = logos::Lexer<Token, S>;

#[cfg(test)]
mod test {
    use super::*;
    use logos::Logos;
    use self::token::Token::*;

    fn assert_lex<T>(source: &str, tokens: T) where T: AsRef<[(Token, &'static str)]> {
        let mut lex = Token::lexer(source);

        for &(ref token, slice) in tokens.as_ref() {
            assert!(
                lex.token == *token && lex.slice() == slice,
                "Expected {:?}({:?}), found {:?}({:?}) instead!\n", token, slice, lex.token, lex.slice()
            );
            lex.advance();
        }

        assert_eq!(lex.token, EndOfFile);
    }

    #[test]
    fn empty_lexer() {
        assert_lex("", []);
        assert_lex("   ", []);
        assert_lex(
            "
            ", &[(Separator, "\n")]);
        assert_lex(
            "

            ",
            &[
                (Separator, "\n"),
                (Separator, "\n")
            ]);
    }

    #[test]
    fn line_comment() {
        assert_lex("-- foo\nbar", [(Comment, "-- foo"), (Separator, "\n"), (Identifier, "bar")]);
        assert_lex("-- foo -- baz\nbar", [(Comment, "-- foo -- baz"), (Separator, "\n"), (Identifier, "bar")]);
        assert_lex("-- foo -- baz \nbar", [(Comment, "-- foo -- baz "), (Separator, "\n"), (Identifier, "bar")]);
        assert_lex("-- foo \" ' _ \nbar", [(Comment, "-- foo \" ' _ "), (Separator, "\n"), (Identifier, "bar")]);

        assert_lex("-- foo ; bar", [(Comment, "-- foo ; bar")]);
        assert_lex("-- foo -- baz ; bar", [(Comment, "-- foo -- baz ; bar")]);
        assert_lex("-- foo -- baz ;\nbar", [(Comment, "-- foo -- baz ;"), (Separator, "\n"), (Identifier, "bar")]);
    }


    #[test]
    fn string_literal() {
        assert_lex("\"abc\"", [(StringLit, "\"abc\"")]);
    }

    #[test]
    fn symbols() {
        assert_lex(
        "( ) [ ] { } + - * /", 
        &[
            (OpenParen, "("), (CloseParen, ")"),
            (OpenBracket, "["), (CloseBracket, "]"),
            (OpenBrace, "{"), (CloseBrace, "}"),
            (Plus, "+"),
            (Minus, "-"),
            (Asterisk, "*"),
            (Slash, "/"),
        ]);
    }
}