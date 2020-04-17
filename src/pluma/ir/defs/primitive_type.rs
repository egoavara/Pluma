use nom::lib::std::iter::Peekable;

use crate::pluma::{PlumaError, Syntax};
use crate::pluma::token::{Keyword, Token, TokenConsumer, TokenStream};
use nom::lib::std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum PrimitiveType {
    I8,
    I16,
    I32,
    I64,
    // I128,
    U8,
    U16,
    U32,
    U64,
    // U128,
    // F16,
    F32,
    F64,
    // F80,
    // F128,
    Size,
    Ptr,
}

impl TokenConsumer for PrimitiveType {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let res = tks.check(|tk| {
            if let Some(Token::Keyword(key)) = tk {
                match key {
                    Keyword::I32 => Ok(PrimitiveType::I32),
                    Keyword::I64 => Ok(PrimitiveType::I64),
                    _ => Err(PlumaError::Syntax(Syntax::UnexpectedToken))
                }
            } else {
                Err(PlumaError::Syntax(Syntax::UnexpectedToken))
            }
        });
        tks.whitespace_or();
        res
    }
}