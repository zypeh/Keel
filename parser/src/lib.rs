extern crate toolshed;
extern crate keel_scanner as scanner;
extern crate keel_ast as ast;

mod error;
mod core;

use toolshed::{Arena, list::GrowableList};
use scanner::{Lexer, Token};
use error::Error;
use ast::*;

pub struct Parser<'ast> {
    arena: &'ast Arena,

    /// Lexer will produce tokens from the source
    lexer: Lexer<&'ast str>,

    // Errors occurred during parsing
    errors: Vec<Error>,

    // AST under construction
    body: ExpressionList<'ast>,
}

impl<'ast> Parser<'ast> {
    pub fn new(source: &'ast str, arena: &'ast Arena) -> Self {
        Parser {
            arena,
            lexer: Lexer::new(source),
            errors: Vec::new(),
            body: NodeList::empty(),
        }
    }

    #[inline]
    fn allow(&mut self, token: Token) -> bool {
        if self.lexer.token == token {
            self.lexer.advance();
            true
        } else {
            false
        }
    }

    #[inline]
    fn expect(&mut self, token: Token) {
        if self.lexer.token == token {
            self.lexer.advance();
        } else {
            self.error();
        }
    }

    #[inline]
    fn expect_end(&mut self, token: Token) -> u32 {
        let end = self.lexer.range().end as u32;
        self.expect(token);
        end
    }

    #[inline]
    fn loc(&mut self) -> (u32, u32) {
        let range = self.lexer.range();
        (range.start as u32, range.end as u32)
    }

    #[inline]
    fn start_then_advance(&mut self) -> u32 {
        let start = self.lexer.range().start as u32;
        self.lexer.advance();
        start
    }

    #[inline]
    fn end_then_advance(&mut self) -> u32 {
        let end = self.lexer.range().end as u32;
        self.lexer.advance();
        end
    }
    
    fn error(&mut self) {
        let token = self.lexer.token.to_owned();
        let raw   = self.lexer.slice().into();
        let span  = self.lexer.range();

        self.errors.push(Error {
            token,
            raw,
            span,
        });
    }

    #[inline]
    fn alloc<T>(&mut self, val: NodeValue<T>) -> Node<'ast, T>
    where
        T: Copy,
    {
        Node::new(self.arena.alloc(val))
    }

    #[inline]
    fn node_at<T, I, R>(&mut self, start: u32, end: u32, item: I) -> R
    where
        T: 'ast + Copy,
        I: Into<T>,
        R: From<Node<'ast, T>>,
    {
        From::from(self.alloc(NodeValue::new(start, end, item.into())))
    }

    #[inline]
    fn node_from_slice<T, F, I, R>(&mut self, func: F) -> R
    where
        T: 'ast + Copy,
        F: FnOnce(&'ast str) -> I,
        I: Into<T>,
        R: From<Node<'ast, T>>,
    {
        let slice = self.lexer.slice();
        let (start, end) = self.loc();

        self.lexer.advance();

        self.node_at(start, end, func(slice))
    }

    #[inline]
    fn comment_slice<T, F, I, R>(&mut self, func: F) -> R
    where
        T: 'ast + Copy,
        F: FnOnce(&'ast str) -> I,
        I: Into<T>,
        R: From<Node<'ast, T>>,
    {
        let slice = self.lexer.slice();
        let (start, end) = self.loc();

        self.lexer.advance();

        // comment starts with "--"
        self.node_at(start + 2, end, func(slice))
    }
}

/// Parse source code from `&str` and produce AST from it.
pub fn parse<'src, 'ast>(source: &'src str) -> Result<Program<'ast>, Vec<Error>> {
    let arena = Arena::new();

    let (body, errors) = {
        let mut parser = Parser::new(source, &arena);

        parser.parse();

        (parser.body.into_unsafe(), parser.errors)
    };

    match errors.len() {
        0 => Ok(Program::new(body, arena)),
        _ => Err(errors)
    }
}