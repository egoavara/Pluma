use nom::bytes::complete::{take_while, take_while_m_n};
use nom::combinator::{map, recognize, verify};
use nom::complete::take;
use nom::IResult;
use nom::sequence::pair;

use crate::pluma::token::{NomMatcher, TokenError, TokenConsumer, TokenStream, Token};

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd, Default)]
pub struct Identifier(String);

fn is_ident_char(c: char) -> bool {
    match c {
        '_' => true,
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

fn is_ident_while_char(c: char) -> bool {
    match c {
        '0'..='9' => true,
        c => is_ident_char(c),
    }
}

impl NomMatcher for Identifier {
    fn nom(src: &str) -> IResult<&str, Self, TokenError> {
        map(
            recognize(
                pair(
                    take_while_m_n(1, 1, is_ident_char),
                    take_while(is_ident_while_char),
                )
            ),
            |x: &str| {
                Identifier(x.to_string())
            },
        )(src)
    }
}


impl TokenConsumer for Identifier {
    type ERR = ();

    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR> {
        let res = tks.check(|tk| {
            if let Some(Token::Identifier(id)) = tk {
                Ok(id.clone())
            }else{
                Err(())
            }
        })?;
        tks.whitespace_or();
        Ok(res)
    }
}