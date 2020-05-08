use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::IResult;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, preceded};

use crate::pluma::token::{Control, Keyword, NomTrait, opt_ws, PlumaError, ws};
use crate::pluma::ir::commons::Value;


//
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Void,
    Primitive(Primitive),
    String,
    Function(Box<Function>),
    Array(Box<Type>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Primitive {
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
    Size,
    Ptr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Function {
    pub params: Vec<Parameter>,
    pub ret: Type,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Parameter(Value, Type);

impl Default for Type {
    fn default() -> Self {
        Type::Void
    }
}

impl Function {
    fn nom_without_fn<'t>(i: &'t str) -> IResult<&'t str, Function, PlumaError<'t>> {
        map(
            pair(
                delimited(
                    pair(
                        Control::GroupStart.matcher(),
                        opt_ws,
                    ),
                    separated_list(
                        delimited(opt_ws, Control::Comma.matcher(), opt_ws),
                        Parameter::nom,
                    ),
                    pair(
                        opt_ws,
                        Control::GroupEnd.matcher(),
                    ),
                ),
                opt(
                    preceded(
                        delimited(opt_ws, Control::Arrow.matcher(), opt_ws),
                        Type::nom,
                    )
                ),
            ),
            |(params, ret)| {
                Function {
                    params,
                    ret: ret.unwrap_or(Type::Void),
                }
            },
        )(i)
    }
}

impl Parameter {
    pub fn new(v: Value, t: Type) -> Self {
        Self(v, t)
    }
}

impl<'t> NomTrait<'t> for Type {
    fn nom(i: &str) -> IResult<&str, Self, PlumaError> {
        alt((
            map(Primitive::nom, |x| Type::Primitive(x)),
            map(Function::nom, |x| Type::Function(Box::new(x))),
            map(Keyword::Str.matcher(), |_|Type::String),
            map(
                preceded(
                    pair(Control::ArrayStart.matcher(), Control::ArrayEnd.matcher()),
                    Type::nom,
                ),
                |x| Type::Array(Box::new(x)),
            ),
        ))(i)
    }
}

impl<'t> NomTrait<'t> for Primitive {
    fn nom(i: &str) -> IResult<&str, Self, PlumaError> {
        Keyword::any_matcher()(i)
            .and_then(|(left, key)| {
                match key {
                    Keyword::I8 => Ok((left, Primitive::I8)),
                    Keyword::I16 => Ok((left, Primitive::I16)),
                    Keyword::I32 => Ok((left, Primitive::I32)),
                    Keyword::I64 => Ok((left, Primitive::I64)),
                    Keyword::U8 => Ok((left, Primitive::U8)),
                    Keyword::U16 => Ok((left, Primitive::U16)),
                    Keyword::U32 => Ok((left, Primitive::U32)),
                    Keyword::U64 => Ok((left, Primitive::U64)),
                    Keyword::F32 => Ok((left, Primitive::F32)),
                    Keyword::F64 => Ok((left, Primitive::F64)),
                    Keyword::Ptr => Ok((left, Primitive::Ptr)),
                    Keyword::Size => Ok((left, Primitive::Size)),
                    _ => Err(nom::Err::Error(PlumaError::NotPrimitive))
                }
            })
    }
}

impl<'t> NomTrait<'t> for Value {
    fn nom(i: &str) -> IResult<&str, Self, PlumaError> {
        alt((
            map(Keyword::Var.matcher(), |_| Value::Variable),
            map(Keyword::Const.matcher(), |_| Value::Constant),
            map(Keyword::Ref.matcher(), |_| Value::Reference),
        ))(i)
    }
}

impl<'t> NomTrait<'t> for Parameter {
    fn nom(i: &str) -> IResult<&str, Self, PlumaError> {
        map(
            pair(
                Value::nom,
                preceded(
                    ws,
                    Type::nom,
                ),
            ),
            |(v, t)| Parameter(v, t),
        )(i)
    }
}

impl<'t> NomTrait<'t> for Function {
    fn nom(i: &str) -> IResult<&str, Self, PlumaError> {
        preceded(
            pair(
                Keyword::Fn.matcher(),
                opt_ws,
            ),
            Function::nom_without_fn,
        )(i)
    }
}