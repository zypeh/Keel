use toolshed::list::ListBuilder;
use Parser;
use ast::*;
use scanner::{Token, lookup, Logos};

type HandlerFn = for<'ast> fn(&mut Parser<'ast>) -> Option<ExpressionNode<'ast>>;
type IsTuple = bool;

static LOOKUP_TABLE: [HandlerFn; Token::SIZE] = lookup! {
    Token::Identifier      => |par| par.node_from_slice(|ident| ident),
    Token::ReservedKeyword => |par| par.node_from_slice(|ident| ident),
    Token::OpenParen       => |par| par.parse_paren(),
    Token::OpenBracket     => |par| par.parse_bracket(),
    Token::StringLit       => |par| par.node_from_slice(|slice| Primitive::String(slice)),
    // Token::CharLit         => |par| par.node_from_slice(|slice| Primitive::Char(slice)), // Need to do validations
    Token::Comment         => |par| par.comment_slice  (|slice| Primitive::Comment(slice)),
    Token::IntegerLit      => |par| par.integer_number(),
    _                      => | _ | None,
};

impl<'ast> Parser<'ast> {
    #[inline]
    pub fn parse(&mut self) {
    }

    #[inline]
    pub fn expression_list(&mut self) -> (IsTuple, ExpressionList<'ast>) {
        let mut is_tuple = false;
        let builder = match LOOKUP_TABLE[self.lexer.token as usize](self) {
            Some(expression) => ListBuilder::new(self.arena, expression),
            None             => return (is_tuple, NodeList::empty()),
        };

        while self.allow(Token::Comma) {
            is_tuple = true;
            match LOOKUP_TABLE[self.lexer.token as usize](self) {
                Some(expression) => builder.push(self.arena, expression),
                None             => self.error(),
            }
        }
        (is_tuple, builder.as_list())
    }

    fn parse_paren(&mut self) -> Option<ExpressionNode<'ast>> {
        let start                   = self.start_then_advance();
        let end                     = self.expect_end(Token::CloseParen);
        let (is_tuple, expressions) = self.expression_list();

        match is_tuple {
            true  => self.node_at(start, end, TupleExpression   { expressions    }),
            false => self.node_at(start, end, GroupedExpression { expression: expressions.first_element().unwrap().clone() })
        }
    }

    fn parse_bracket(&mut self) -> Option<ExpressionNode<'ast>> {
        let start            = self.start_then_advance();
        let end              = self.expect_end(Token::CloseBracket);
        let (_, expressions) = self.expression_list();
        self.node_at(start, end, ArrayExpression { expressions })
    }

    fn integer_number(&mut self) -> Option<ExpressionNode<'ast>> {
        let number = self.lexer.slice();
        let (start, end) = self.loc();
        self.lexer.advance();
        self.node_at(start, end, Primitive::IntegerNumber(number))
    }
}