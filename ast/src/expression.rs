use {*};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expression<'ast> {
    IdentifierExpression(Identifier<'ast>),
    PrimitiveExpression(Primitive<'ast>),
    ArrayExpression(ArrayExpression<'ast>),
    IndexAccessExpression(IndexAccessExpression<'ast>),
    GroupedExpression(GroupedExpression<'ast>),
    TupleExpression(TupleExpression<'ast>),
    ClosureExpression(ClosureExpression<'ast>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Primitive<'ast> {
    String(&'ast str),
    Char(&'ast char),
    Comment(&'ast str),
    IntegerNumber(&'ast str),
}

// [a, b, c]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrayExpression<'ast> {
    pub expressions: ExpressionList<'ast>,
}

// <array>[<index>]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndexAccessExpression<'ast> {
    pub array: ExpressionNode<'ast>,
    pub index: Option<ExpressionNode<'ast>>,
}

// (a b c) 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GroupedExpression<'ast> {
    pub expression: ExpressionNode<'ast>
}

// (a, b, c)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TupleExpression<'ast> {
    pub expressions: ExpressionList<'ast>,
}

// \x => ...
// \x , y => ... -- accept two arguments
// \(x, y, z) => ... -- accept only one arguments, tuple (x, y, z)
// \x => { ... }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClosureExpression<'ast> {
    pub parameters: ParameterList<'ast>,
    pub block: ExpressionList<'ast>,
    pub returns: ExpressionNode<'ast>,
}

impl_from! {
    Identifier => Expression::IdentifierExpression,
    Primitive => Expression::PrimitiveExpression,
    ArrayExpression => Expression::ArrayExpression,
    GroupedExpression => Expression::GroupedExpression,
    TupleExpression => Expression::TupleExpression,
    IndexAccessExpression => Expression::IndexAccessExpression,
}