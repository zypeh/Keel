extern crate toolshed;

#[macro_use]
mod impl_from;
mod node;
mod statement;
mod expression;

use toolshed::{
    Arena,
    list::List,
    list::UnsafeList
};
use std::marker::PhantomData;

use expression::Expression;
use statement::Parameter;

pub use self::node::{Node, NodeValue};
pub use self::expression::*;
pub use self::statement::*;

pub type Identifier<'ast> = &'ast str;
pub type StringLiteral<'ast> = &'ast str;
pub type CharLiteral<'ast> = &'ast char;

pub type NodeList<'ast, T> = List<'ast, Node<'ast, T>>;

pub type IdentifierNode<'ast> = Node<'ast, Identifier<'ast>>;
pub type IdentifierList<'ast> = NodeList<'ast, Identifier<'ast>>;

// pub type SourceUnit<'ast> = Node<'ast, SourceFile<'ast>>;
pub type ExpressionNode<'ast> = Node<'ast, Expression<'ast>>;
pub type ExpressionList<'ast> = NodeList<'ast, Expression<'ast>>;

pub type ParameterNode<'ast> = Node<'ast, Parameter<'ast>>;
pub type ParameterList<'ast> = NodeList<'ast, Parameter<'ast>>;

/// AST
pub struct Program<'ast> {
    body: UnsafeList,
    /// `Arena` on which the entire AST is allocated.
    arena: Arena,
    _phantom: PhantomData<ExpressionList<'ast>>
}

impl<'ast> Program<'ast> {
    #[inline]
    pub fn new(body: UnsafeList, arena: Arena) -> Self {
        Program {
            body,
            arena,
            _phantom: PhantomData,
        }
    }

    /// Get the list of `SourceUnit`s.
    #[inline]
    pub fn body(&self) -> ExpressionList<'ast> {
        unsafe { self.body.into_list() }
    }

    /// Get a reference to the `Arena` on which the AST is allocated.
    #[inline]
    pub fn arena(&'ast self) -> &'ast Arena {
        &self.arena
    }
}