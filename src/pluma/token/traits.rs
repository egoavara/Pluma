use std::collections::VecDeque;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::pluma::token::Token;

pub trait TokenConsumer: Sized {
    type ERR;
    fn consume(tks: &mut TokenStream) -> Result<Self, Self::ERR>;
}

pub struct TokenStream {
    raw: Vec<Token>,
    start_at: usize,
}

impl TokenStream {
    pub fn new<II: IntoIterator<Item=Token>>(i: II) -> Self {
        Self {
            raw: i.into_iter().collect(),
            start_at: 0,
        }
    }
    pub fn whitespace_or(&mut self) -> &mut Self {
        self.require(|x| {
            if let Some(Token::Whitespace) = x {
                Ok(())
            } else {
                Err(())
            }
        });
        self
    }
    pub fn whitespace<ERR, GERRF: FnOnce() -> ERR>(&mut self, e: GERRF) -> Result<&mut Self, ERR> {
        self.require(|x| {
            if let Some(Token::Whitespace) = x {
                Ok(())
            } else {
                Err(e())
            }
        })
    }
    pub fn require<ERR, F: FnOnce(Option<&Token>) -> Result<(), ERR>>(&mut self, f: F) -> Result<&mut Self, ERR> {
        f(self.raw.get(self.start_at))?;
        self.start_at += 1;
        Ok(self)
    }
    pub fn check<OK, ERR, F: FnOnce(Option<&Token>) -> Result<OK, ERR>>(&mut self, f: F) -> Result<OK, ERR> {
        let tmp = f(self.raw.get(self.start_at));
        if tmp.is_ok() {
            self.start_at += 1;
        }
        tmp
    }
    pub fn pick<TC : TokenConsumer>(&mut self) -> Result<TC, TC::ERR>{ TC::consume(self) }
    pub fn opt_pick<TC : TokenConsumer>(&mut self) -> Option<TC>{
        TC::consume(self).ok()
    }

    pub fn direct(&mut self) -> (&mut Vec<Token>, usize) {
        (&mut self.raw, self.start_at)
    }
}