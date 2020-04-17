use crate::pluma::token::{TokenConsumer, TokenStream, Token, Keyword};
use crate::pluma::{PlumaError, Syntax};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Accessor {
    Variable,
    Constant,
    Reference,
}
impl Default for Accessor{
    fn default() -> Self {
        Accessor::Variable
    }
}

impl TokenConsumer for Accessor {
    type ERR = PlumaError;

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let res = tks.check(|x| {
            if let Some(Token::Keyword(key)) = x {
                match key {
                    Keyword::Var => Ok(Accessor::Variable),
                    Keyword::Const => Ok(Accessor::Constant),
                    Keyword::Ref => Ok(Accessor::Reference),
                    _ => Err(PlumaError::Syntax(Syntax::UnexpectedToken)),
                }
            } else {
                Err(PlumaError::Syntax(Syntax::UnexpectedToken))
            }
        })?;
        tks.whitespace_or();
        Ok(res)
    }
}