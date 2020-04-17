use crate::pluma::token::{Identifier, Keyword};

pub enum Expression {
    Reference(Identifier),
    // Word(Keyword),
    //
    UnaryExpression(UnaryOperator, Box<Expression>),
    BinaryExpression(BinaryOperator, Box<Expression>, Box<Expression>),
}


impl IntoIterator for Expression {
    type Item = Expression;
    type IntoIter = std::vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Expression::Reference(_) => vec![].into_iter(),
            Expression::UnaryExpression(_, a0) => vec![*a0].into_iter(),
            Expression::BinaryExpression(_, a0, a1) => vec![*a0, *a1].into_iter(),
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BinaryOperator {
    Add
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Pos,
    Neg,
    Not,
}