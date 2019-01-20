use {*};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parameter<'ast> {
    pub type_name: Option<IdentifierNode<'ast>>,
    pub name: Identifier<'ast>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Statement<'ast> {
    ValueBinding(ValueBinding<'ast>),
    FunctionBinding(FunctionBinding<'ast>),
}

// let name : type = values
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValueBinding<'ast> {
    pub name: ParameterNode<'ast>,
    pub value: ExpressionNode<'ast>,
}

// TODO: desugar pattern matching in let-assignment
// // deconstruct and assign in let binding
// // let [a', b', c'] = [a, b, c]
// // let (a', b', c') = (a, b, c)
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub struct MultipleValueBinding<'ast> {
//     pub names: ParameterList<'ast>>,
//     pub value: ExpressionList<'ast>>,
// }

// let name x y = ...
// let name (x: type1) (y: type2) : type3 = ...
// let (x: type1) <symbol> (y: type2) : type3 = ... -- an infix operator
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FunctionBinding<'ast> {
    pub name: IdentifierNode<'ast>,
    pub is_infix: bool,
    pub parameters: ParameterList<'ast>,
    pub block: ExpressionList<'ast>,
    pub returns: ParameterList<'ast>,
}

impl_from! {
    ValueBinding => Statement::ValueBinding,
    FunctionBinding => Statement::FunctionBinding,
}