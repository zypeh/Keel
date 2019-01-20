extern crate toolshed;

#[macro_use]
mod impl_from;
mod node;
mod statement;
mod expression;

use toolshed::list::{List};
use expression::Expression;
use statement::Parameter;

pub use self::node::{Node};

pub type Identifier<'ast> = &'ast str;
pub type StringLiteral<'ast> = &'ast str;
pub type CharLiteral<'ast> = &'ast char;

pub type NodeList<'ast, T> = List<'ast, Node<'ast, T>>;

pub type IdentifierNode<'ast> = Node<'ast, Identifier<'ast>>;
pub type IdentifierList<'ast> = NodeList<'ast, Identifier<'ast>>;

pub type ExpressionNode<'ast> = Node<'ast, Expression<'ast>>;
pub type ExpressionList<'ast> = NodeList<'ast, Expression<'ast>>;

pub type ParameterNode<'ast> = Node<'ast, Parameter<'ast>>;
pub type ParameterList<'ast> = NodeList<'ast, Parameter<'ast>>;