use nom::lib::std::iter::Peekable;

pub use primitive_type::*;
pub use function_type::*;
use crate::pluma::token::{TokenConsumer, TokenStream};
use crate::pluma::PlumaError;


mod primitive_type;
mod function_type;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Void,
    Primitive(PrimitiveType),
    Function(Box<FunctionType<NoIdentParam>>),
}

impl TokenConsumer for Type {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        unimplemented!()
    }
}