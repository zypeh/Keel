use {*};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expression<'ast> {
    IdentifierExpression(Identifier<'ast>),
    ArrayExpression(ArrayExpression<'ast>),
    IndexAccessExpression(IndexAccessExpression<'ast>),
    TupleExpression(TupleExpression<'ast>),
    ConditionalExpression(ConditionalExpression<'ast>),
    ClosureExpression(ClosureExpression<'ast>),
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

// (a, b, c)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TupleExpression<'ast> {
    pub expressions: ExpressionList<'ast>,
}

// if <predicate> then <consequent> else <alternate>
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConditionalExpression<'ast> {
    pub predicate: ExpressionNode<'ast>,
    pub consequent: ExpressionNode<'ast>,
    pub alternate: Option<ExpressionNode<'ast>>,
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
    ArrayExpression => Expression::ArrayExpression,
    TupleExpression => Expression::TupleExpression,
    IndexAccessExpression => Expression::IndexAccessExpression,
    ConditionalExpression => Expression::ConditionalExpression,
}