use crate::pluma::ir::{Type, Accessor};
use crate::pluma::token::{Identifier, TokenConsumer, Token, TokenStream};
use nom::lib::std::iter::Peekable;
use crate::pluma::{PlumaError, Syntax};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FunctionType<PARAM : Param> {
    ret: Type,
    params: Vec<PARAM>,
}
pub trait Param : TokenConsumer{

}

pub struct NoIdentParam(Accessor, Type);

impl Param for NoIdentParam{

}

impl TokenConsumer for NoIdentParam {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let acc = tks.opt_pick::<Accessor>().unwrap_or(Default::default());
        let tp = tks.pick::<Type>()?;
        Ok(Self(acc, tp))
    }
}

pub struct IdentParam(Accessor, Identifier, Type);

impl Param for IdentParam{

}

impl TokenConsumer for IdentParam {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let acc = tks.opt_pick::<Accessor>().unwrap_or(Default::default());
        let idnt = tks.pick::<Identifier>().map_err(|_|PlumaError::Syntax(Syntax::UnexpectedToken))?;
        let tp = tks.pick::<Type>()?;
        Ok(Self(acc, idnt, tp))
    }
}