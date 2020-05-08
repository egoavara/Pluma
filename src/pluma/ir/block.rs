use crate::pluma::token::{NomTrait, PlumaError, Control, ws, sep};
use nom::IResult;
use nom::sequence::{delimited, pair};
use nom::combinator::{opt, map};
use nom::multi::separated_list;

#[derive(Debug, Clone)]
pub struct Block {
    exprs : Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {}


impl<'t> NomTrait<'t> for Block {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        map(
            delimited(
                pair(Control::BlockStart.matcher(), opt(sep)),
                separated_list(
                    sep,
                    Expression::nom,
                ),
                pair(opt(sep), Control::BlockEnd.matcher()),
            ),
            |x|{
                Block{
                    exprs : x,
                }
            }
        )(i)
    }
}

impl<'t> NomTrait<'t> for Expression {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        Err(nom::Err::Error(PlumaError::Unknown))
    }
}