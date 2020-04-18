use crate::pluma::ir::{Type, Accessor};
use crate::pluma::token::{Identifier, TokenConsumer, Token, TokenStream, Keyword, Control};
use nom::lib::std::iter::Peekable;
use crate::pluma::{PlumaError, Syntax};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FunctionType<PARAM: Param> {
    ret: Type,
    params: Vec<PARAM>,
}

impl<PARAM: Param> TokenConsumer for FunctionType<PARAM> {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        tks.keyword(&Keyword::Fn, ||PlumaError::Syntax(Syntax::ExpectedToken(Token::Keyword(Keyword::Fn))));
        tks.whitespace_or();
        tks.control(&Control::GroupStart, ||PlumaError::Syntax(Syntax::ExpectedToken(Token::Control(Control::GroupStart))));
        tks.whitespace_or();
        tks.control(&Control::GroupEnd, ||PlumaError::Syntax(Syntax::ExpectedToken(Token::Control(Control::GroupEnd))));
        tks.whitespace_or();
        let ret = tks.opt_pick::<Type>();

        //
        Ok(Self{
            ret: ret.unwrap_or(Type::Void),
            params: vec![],
        })
    }
}


pub trait Param: TokenConsumer {
    fn get_accessor(&self) -> &Accessor;
    fn get_identifier(&self) -> Option<&Identifier>;
    fn get_type(&self) -> &Type;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NoIdentParam(Accessor, Type);

impl Param for NoIdentParam {
    fn get_accessor(&self) -> &Accessor { &self.0 }

    fn get_identifier(&self) -> Option<&Identifier> { None }

    fn get_type(&self) -> &Type { &self.1 }
}

impl TokenConsumer for NoIdentParam {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let acc = tks.opt_pick::<Accessor>().unwrap_or(Default::default());
        let tp = tks.pick::<Type>()?;
        Ok(Self(acc, tp))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentParam(Accessor, Identifier, Type);

impl Param for IdentParam {
    fn get_accessor(&self) -> &Accessor {
        &self.0
    }

    fn get_identifier(&self) -> Option<&Identifier> {
        Some(&self.1)
    }

    fn get_type(&self) -> &Type {
        &self.2
    }
}

impl TokenConsumer for IdentParam {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let acc = tks.opt_pick::<Accessor>().unwrap_or(Default::default());
        let idnt = tks.pick::<Identifier>().map_err(|_| PlumaError::Syntax(Syntax::UnexpectedToken))?;
        let tp = tks.pick::<Type>()?;
        Ok(Self(acc, idnt, tp))
    }
}

impl From<IdentParam> for NoIdentParam {
    fn from(iprm: IdentParam) -> Self {
        NoIdentParam(iprm.0, iprm.2)
    }
}