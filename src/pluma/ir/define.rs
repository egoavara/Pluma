use nom::combinator::{map, opt};
use nom::IResult;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

use crate::pluma::ir::block::Block;
use crate::pluma::ir::commons::Value;
use crate::pluma::ir::types::{Parameter, Type};
use crate::pluma::ir::types;
use crate::pluma::token::{Control, GlobalIdentifier, Identifier, Keyword, NomTrait, opt_ws, PlumaError, ws};
use nom::branch::alt;

#[derive(Debug, Clone)]
pub enum Define {
    Function(Function),
    Static(Static),
}

#[derive(Debug, Clone)]
pub struct Function {
    name: Identifier,
    declare: types::Function,
    params: Vec<Identifier>,
    define: Block,
}
#[derive(Debug, Clone)]
pub struct Static {
    value: Value,
    name: Identifier,
    declare: Type,
    define: Block,
}


impl<'t> NomTrait<'t> for Define {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        alt((
            map(Function::nom, |x| Define::Function(x)),
            map(Static::nom, |x| Define::Static(x)),
        ))(i)
    }
}

impl<'t> NomTrait<'t> for Function {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        map(
            preceded(
                // 'fn '
                pair(Keyword::Fn.matcher(), ws),
                pair(
                    pair(
                        terminated(Identifier::nom, opt(ws)),
                        delimited(
                            pair(Control::GroupStart.matcher(), opt_ws),
                            separated_list(
                                tuple((opt_ws, Control::Comma.matcher(), opt_ws)),
                                map(
                                    pair(
                                        opt(terminated(Value::nom, ws)),
                                        pair(
                                            terminated(Identifier::nom, ws),
                                            Type::nom,
                                        ),
                                    ),
                                    |(ov, (ident, tp))| {
                                        (ov.unwrap_or_default(), ident, tp)
                                    },
                                ),
                            ),
                            pair(opt_ws, Control::GroupEnd.matcher()),
                        ),
                    ),
                    pair(
                        opt(preceded(tuple((opt_ws, Control::Arrow.matcher(), opt_ws)), Type::nom)),
                        Block::nom,
                    ),
                ),
            ),
            |((ident, params), (ret, block))| {
                let (dec, names) = params.into_iter().fold((Vec::new(), Vec::new()), |(mut a, mut b), (v, n, t)| {
                    a.push(Parameter::new(v, t));
                    b.push(n);
                    (a, b)
                });
                Function {
                    name: ident,
                    declare: types::Function {
                        params: dec,
                        ret: ret.unwrap_or_default(),
                    },
                    params: names,
                    define: block,
                }
            },
        )(i)
    }
}

impl<'t> NomTrait<'t> for Static {
    fn nom(i: &'t str) -> IResult<&'t str, Self, PlumaError<'t>> {
        map(
            pair(
                pair(
                    terminated(Value::nom, ws),
                    terminated(Identifier::nom, ws),
                ),
                pair(
                    Type::nom,
                    preceded(
                        tuple((opt_ws, Control::Equal.matcher(), opt_ws)),
                        Block::nom,
                    ),
                ),
            ),
            |((v, i), (t, b))| {
                Static {
                    value: v,
                    name: i,
                    declare: t,
                    define: b,
                }
            },
        )(i)
    }
}

