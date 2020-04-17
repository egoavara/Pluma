use std::io::{BufRead, BufReader, Cursor, Error, Read};
use std::ops::Index;

use nom::{AsChar, InputIter, IResult};
use nom::bits::complete::take;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while, take_while1};
use nom::character::complete::char;
use nom::character::is_space;
use nom::combinator::{all_consuming, cond, map, opt, recognize, verify};
use nom::error::ErrorKind;
use nom::lib::std::borrow::Borrow;
use nom::lib::std::convert::{TryFrom, TryInto};
use nom::lib::std::fmt::Debug;
use nom::lib::std::str::FromStr;
use nom::multi::{fold_many0, many0, separated_list, separated_nonempty_list};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::Slice;

pub use identifier::*;
pub use traits::*;

mod traits;
mod identifier;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Token {
    Whitespace,
    Keyword(Keyword),
    Control(Control),

    Identifier(Identifier),

    Bytes(Vec<u8>),
    Numeric(String),
    Literal(String),

    GroupStart,
    GroupEnd,
    BlockStart,
    BlockEnd,
    ArrayStart,
    ArrayEnd,
    GenericStart,
    GenericEnd,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Keyword {
    If,
    Else,
    Is,
    With,

    And,
    Or,
    As,

    Func,
    Struct,
    Interface,

    Import,
    Package,
    Var,
    Const,
    Ref,
    For,
    In,

    // Primitives
    I32,
    I64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Control {
    Plus,
    Equal,
    Minus,

    Not,
    Sharp,
    Mod,
    Pow,
    And,
    Mul,

    Semicolon,
    Comma,
    Dot,
    Question,
    Slash,
    Or,
    Meta,
}

pub trait NomMatcher: Sized {
    fn nom(src: &str) -> IResult<&str, Self, TokenError>;
}

#[derive(Debug)]
pub enum TokenError {
    UnknownControl(String),
    UnknownInput(String),
    IOError(Error),
}


impl<I: Debug + AsRef<str>> nom::error::ParseError<I> for TokenError {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        // println!("{:?}", kind);
        TokenError::UnknownInput(input.as_ref().to_string())
    }

    fn append(input: I, kind: ErrorKind, other: Self) -> Self {
        // println!("  {:?}", kind);
        TokenError::UnknownInput(input.as_ref().to_string())
    }

    fn add_context(_input: I, _ctx: &'static str, other: Self) -> Self {
        match _ctx {
            "control" => TokenError::UnknownControl(_input.as_ref().to_string()),
            _ => other,
        }
    }
}

impl NomMatcher for Keyword {
    fn nom(src: &str) -> IResult<&str, Self, TokenError> {
        alt((
            alt((
                map(tag("if"), |_| Keyword::If),
                map(tag("else"), |_| Keyword::Else),
                map(tag("is"), |_| Keyword::Is),
                map(tag("in"), |_| Keyword::In),
                map(tag("with"), |_| Keyword::With),
                map(tag("and"), |_| Keyword::And),
                map(tag("as"), |_| Keyword::As),
                map(tag("or"), |_| Keyword::Or),
                map(tag("func"), |_| Keyword::Func),
                map(tag("struct"), |_| Keyword::Struct),
                map(tag("interface"), |_| Keyword::Interface),
                map(tag("import"), |_| Keyword::Import),
                map(tag("package"), |_| Keyword::Package),
                map(tag("var"), |_| Keyword::Var),
                map(tag("const"), |_| Keyword::Const),
                map(tag("ref"), |_| Keyword::Ref),
                map(tag("for"), |_| Keyword::For),
            )),
            alt((
                map(tag("i32"), |_| Keyword::I32),
                map(tag("i64"), |_| Keyword::I64),
            )),
        ))(src)
    }
}

impl NomMatcher for Control {
    fn nom(src: &str) -> IResult<&str, Self, TokenError> {
        alt((
            map(tag("+"), |_| Control::Plus),
            map(tag("="), |_| Control::Plus),
            map(tag("-"), |_| Control::Minus),
            map(tag("!"), |_| Control::Not),
            map(tag("#"), |_| Control::Sharp),
            map(tag("%"), |_| Control::Mod),
            map(tag("^"), |_| Control::Pow),
            map(tag("&"), |_| Control::And),
            map(tag("*"), |_| Control::Mul),
            map(tag(";"), |_| Control::Semicolon),
            map(tag(","), |_| Control::Comma),
            map(tag("."), |_| Control::Dot),
            map(tag("?"), |_| Control::Question),
            map(tag("/"), |_| Control::Slash),
            map(tag("|"), |_| Control::Or),
            map(tag(":"), |_| Control::Meta),
        ))(src)
    }
}

fn numeric(src: &str) -> IResult<&str, String, TokenError> {
    alt((
        decimal,
        map(pair(integer, decimal), |(a, b)| a + &b),
        map(pair(integer, tag(".")), |(a, b)| a + b),
        integer,
    ))(src)
}

fn integer(src: &str) -> IResult<&str, String, TokenError> {
    map(
        take_while1(|x: char| x.is_digit(10)),
        |x: &str| { x.to_string() },
    )(src)
}

fn decimal(src: &str) -> IResult<&str, String, TokenError> {
    map(
        preceded(
            tag("."),
            take_while(|x: char| x.is_digit(10)),
        ),
        |x: &str| { ".".to_string() + x },
    )(src)
}

fn literal(src: &str) -> IResult<&str, String, TokenError> {
    delimited(
        tag("\""),
        fold_many0(
            unescaped,
            String::new(),
            |mut s, c| {
                s.push(c);
                s
            },
        ),
        tag("\""),
    )(src)
}

fn unescaped(src: &str) -> IResult<&str, char, TokenError> {
    match src.iter_elements().next().map(|t: char| {
        match t {
            '\\' => None,
            '\"' => None,
            '\'' => None,
            t => Some(t),
        }
    }) {
        Some(Some(c)) => Ok((src.slice(c.len()..), c)),
        Some(None) | _ => Err(nom::Err::Error(TokenError::UnknownInput(src.to_string()))),
    }
}

fn sep(src: &str) -> IResult<&str, (), TokenError> {
    map(
        take_while1(|x: char| {
            match x {
                ';' => true,
                x => x.is_whitespace(),
            }
        }),
        |_| (),
    )(src)
}

impl NomMatcher for Token {
    fn nom(src: &str) -> IResult<&str, Self, TokenError> {
        alt((
            map(sep, |_| Token::Whitespace),
            map(Control::nom, |c| Token::Control(c)),
            map(Keyword::nom, |k| Token::Keyword(k)),
            map(Identifier::nom, |id| Token::Identifier(id)),
            map(numeric, |x| Token::Numeric(x)),
            map(literal, |x| Token::Literal(x)),
            map(tag("("), |_| Token::GroupStart),
            map(tag(")"), |_| Token::GroupEnd),
            map(tag("{"), |_| Token::BlockStart),
            map(tag("}"), |_| Token::BlockEnd),
            map(tag("["), |_| Token::ArrayStart),
            map(tag("]"), |_| Token::ArrayEnd),
            map(tag("<"), |_| Token::GenericStart),
            map(tag(">"), |_| Token::GenericEnd),
        ))(src)
    }
}

impl Token {
    pub fn tokenize_str(s: &str) -> Result<Vec<Token>, TokenError> {
        all_consuming(
            many0(Token::nom),
        )(s)
            .map(|(_, ok)| ok)
            .map_err(|err| {
                match err {
                    nom::Err::Incomplete(_) => unreachable!(),
                    nom::Err::Error(err) => err,
                    nom::Err::Failure(err) => err,
                }
            })
    }
    pub fn tokenize(src: &mut dyn Read) -> Result<Vec<Token>, TokenError> {
        let mut str_src = String::new();
        src.read_to_string(&mut str_src).map_err(|err| TokenError::IOError(err))?;
        Self::tokenize_str(str_src.as_str())
    }
}
