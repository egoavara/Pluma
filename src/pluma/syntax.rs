use crate::pluma::token::Token;

pub type PlumaError = TokenError<Critical, Syntax>;

#[derive(Debug)]
pub enum TokenError<C, S>{
    Critical(C),
    Syntax(S),
}

#[derive(Debug)]
pub enum Critical{
}

#[derive(Debug)]
pub enum Syntax{
    ExpectedToken(Token),
    UnexpectedToken,
}