use nom::{IResult, Needed};
use nom::branch::alt;
use nom::combinator::{map, opt, recognize};
use nom::bytes::complete::{tag, take_till1, take_till, take_while1, take_while};
use nom::error::{ErrorKind, context};
use nom::character::complete::one_of;
use nom::lib::std::str::FromStr;
use nom::sequence::pair;
use nom::multi::separated_list;

#[derive(Debug)]
pub enum PlumaError<'t> {
    Unknown,
    Needed(Needed),
    Left(&'t str),
    //
    NoWhitespace,
    NotKeyword,
    NotExpectedKeyword(Keyword),
    NotControl,
    NotExpectedControl(Control),
    //
    NotPrimitive,
}

impl<'t> nom::error::ParseError<&'t str> for PlumaError<'t> {
    fn from_error_kind(input: &'t str, kind: ErrorKind) -> Self {
        PlumaError::Unknown
    }

    fn append(input: &'t str, kind: ErrorKind, other: Self) -> Self {
        other
    }

    // fn or(self, other: Self) -> Self {
    //     unimplemented!()
    // }

    fn add_context(_input: &'t str, _ctx: &'static str, other: Self) -> Self {
        match _ctx {
            "NotKeyword" => PlumaError::NotKeyword,
            "NotControl" => PlumaError::NotControl,
            _ => other
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Keyword {
    If,
    Else,
    Is,
    With,
    And,
    Or,
    As,
    Fn,
    Struct,
    Interface,
    Import,
    Package,
    Var,
    Const,
    Ref,
    For,
    In,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Ptr,
    Size,
    Str,
}

impl Keyword {
    pub fn any_matcher() -> impl Fn(&str) -> IResult<&str, Keyword, PlumaError> {
        move |i| {
            context(
                "NotKeyword",
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
                        map(tag("fn"), |_| Keyword::Fn),
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
                        map(tag("i8"), |_| Keyword::I8),
                        map(tag("i16"), |_| Keyword::I16),
                        map(tag("i32"), |_| Keyword::I32),
                        map(tag("i64"), |_| Keyword::I64),
                        map(tag("u8"), |_| Keyword::U8),
                        map(tag("u16"), |_| Keyword::U16),
                        map(tag("u32"), |_| Keyword::U32),
                        map(tag("u64"), |_| Keyword::U64),
                        map(tag("f32"), |_| Keyword::F32),
                        map(tag("f64"), |_| Keyword::F64),
                        map(tag("size"), |_| Keyword::Size),
                        map(tag("ptr"), |_| Keyword::Ptr),
                        map(tag("str"), |_| Keyword::Str),
                    )),
                )),
            )(i)
        }
    }
    pub fn matcher(self) -> impl Fn(&str) -> IResult<&str, (), PlumaError> {
        move |i| {
            let (left, key) = Self::any_matcher()(i)?;
            if key == self {
                Ok((left, ()))
            } else {
                Err(nom::Err::Error(PlumaError::NotExpectedKeyword(self)))
            }
        }
    }
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
    //
    Arrow,
    //
    GroupStart,
    GroupEnd,
    BlockStart,
    BlockEnd,
    ArrayStart,
    ArrayEnd,
    GenericStart,
    GenericEnd,
}

impl Control {
    pub fn any_matcher() -> impl Fn(&str) -> IResult<&str, Control, PlumaError> {
        move |i| {
            context(
                "NotControl",
                alt((
                    alt((
                        map(tag("->"), |_| Control::Arrow),
                        map(tag("+"), |_| Control::Plus),
                        map(tag("="), |_| Control::Equal),
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
                    )),
                    alt((
                        map(tag("("), |_| Control::GroupStart),
                        map(tag(")"), |_| Control::GroupEnd),
                        map(tag("{"), |_| Control::BlockStart),
                        map(tag("}"), |_| Control::BlockEnd),
                        map(tag("["), |_| Control::ArrayStart),
                        map(tag("]"), |_| Control::ArrayEnd),
                        map(tag("<"), |_| Control::GenericStart),
                        map(tag(">"), |_| Control::GenericEnd),
                    ))
                )),
            )(i)
        }
    }
    pub fn matcher(self) -> impl Fn(&str) -> IResult<&str, (), PlumaError> {
        move |i| {
            let (left, ctl) = Self::any_matcher()(i)?;
            if ctl == self {
                Ok((left, ()))
            } else {
                Err(nom::Err::Error(PlumaError::NotExpectedControl(self)))
            }
        }
    }
}

pub trait NomTrait<'t>: Sized {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>>;
    fn parse(i: &'t str) -> Result<Self, PlumaError<'t>> {
        Self::nom(i)
            .map_err(|x| {
                match x {
                    nom::Err::Error(err) | nom::Err::Failure(err) => err,
                    nom::Err::Incomplete(need) => PlumaError::Needed(need),
                }
            })
            .and_then(|(left, nt)| {
                if left.is_empty() {
                    Ok(nt)
                } else {
                    Err(PlumaError::Left(left))
                }
            })
    }
}

pub fn opt_ws(i: &str) -> IResult<&str, (), PlumaError> {
    map(opt(ws), |_| ())(i)
}

pub fn ws(i: &str) -> IResult<&str, (), PlumaError> {
    map(one_of(" \t\n"), |_| ())(i)
}

pub fn sep(i: &str) -> IResult<&str, (), PlumaError> {
    map(one_of(";\n"), |_| ())(i)
}

pub fn ws_sep(i: &str) -> IResult<&str, (), PlumaError> {
    alt((
        ws,
        sep,
    ))(i)
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn ident_char(c: char) -> bool {
        match c {
            '_' => true,
            'a'..='z' => true,
            'A'..='Z' => true,
            'ㄱ'..='ㅎ' => true,
            'ㅏ'..='ㅣ' => true,
            '가'..='힣' => true,
            _ => false,
        }
    }
    pub fn ident_char_and_numeric(c: char) -> bool {
        match c {
            '0'..='9' => true,
            c => Self::ident_char(c),
        }
    }
}

impl<'t> NomTrait<'t> for Identifier {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        map(
            recognize(pair(
                take_while1(Self::ident_char),
                take_while(Self::ident_char_and_numeric),
            )),
            |x: &str| {
                Identifier(String::from(x))
            },
        )(i)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct GlobalIdentifier(Vec<Identifier>);

impl<'t> NomTrait<'t> for GlobalIdentifier {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        map(
            separated_list(
                Control::Dot.matcher(),
                Identifier::nom,
            ),
            |x| GlobalIdentifier(x),
        )(i)
    }
}
//
// fn integer(src: &str) -> IResult<&str, String, TokenError> {
//     map(
//         take_while1(|x: char| x.is_digit(10)),
//         |x: &str| { x.to_string() },
//     )(src)
// }
//
// fn decimal(src: &str) -> IResult<&str, String, TokenError> {
//     map(
//         preceded(
//             tag("."),
//             take_while(|x: char| x.is_digit(10)),
//         ),
//         |x: &str| { ".".to_string() + x },
//     )(src)
// }
//
// fn literal(src: &str) -> IResult<&str, String, TokenError> {
//     delimited(
//         tag("\""),
//         fold_many0(
//             unescaped,
//             String::new(),
//             |mut s, c| {
//                 s.push(c);
//                 s
//             },
//         ),
//         tag("\""),
//     )(src)
// }
//
// fn unescaped(src: &str) -> IResult<&str, char, TokenError> {
//     match src.iter_elements().next().map(|t: char| {
//         match t {
//             '\\' => None,
//             '\"' => None,
//             '\'' => None,
//             t => Some(t),
//         }
//     }) {
//         Some(Some(c)) => Ok((src.slice(c.len()..), c)),
//         Some(None) | _ => Err(nom::Err::Error(TokenError::UnknownInput(src.to_string()))),
//     }
// }
//
// fn sep(src: &str) -> IResult<&str, (), TokenError> {
//     map(
//         take_while1(|x: char| {
//             match x {
//                 ';' => true,
//                 x => x.is_whitespace(),
//             }
//         }),
//         |_| (),
//     )(src)
// }
//
// impl NomMatcher for Token {
//     fn nom(src: &str) -> IResult<&str, Self, TokenError> {
//         alt((
//             map(sep, |_| Token::Whitespace),
//             map(Control::nom, |c| Token::Control(c)),
//             map(Keyword::nom, |k| Token::Keyword(k)),
//             map(Identifier::nom, |id| Token::Identifier(id)),
//             map(numeric, |x| Token::Numeric(x)),
//             map(literal, |x| Token::Literal(x)),
//         ))(src)
//     }
// }
//
// impl Token {
//     pub fn tokenize_str(s: &str) -> Result<Vec<Token>, TokenError> {
//         all_consuming(
//             many0(Token::nom),
//         )(s)
//             .map(|(_, ok)| ok)
//             .map_err(|err| {
//                 match err {
//                     nom::Err::Incomplete(_) => unreachable!(),
//                     nom::Err::Error(err) => err,
//                     nom::Err::Failure(err) => err,
//                 }
//             })
//     }
//     pub fn tokenize(src: &mut dyn Read) -> Result<Vec<Token>, TokenError> {
//         let mut str_src = String::new();
//         src.read_to_string(&mut str_src).map_err(|err| TokenError::IOError(err))?;
//         Self::tokenize_str(str_src.as_str())
//     }
// }
