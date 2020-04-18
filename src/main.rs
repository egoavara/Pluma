use std::fs::File;
use std::io::Read;

use nom::bytes::complete::take_while1;
use nom::multi::separated_list;

use crate::pluma::ir::{PrimitiveType, Type};
use crate::pluma::token::{Control, NomMatcher, Token, TokenConsumer, TokenStream};

mod pluma;

fn main() {
    let mut f = File::open("./examples/00_helloworld.pluma").unwrap();

    for tk in Token::tokenize(&mut f).unwrap() {
        println!("{:?}", tk);
    }

    let mut tks = TokenStream::new(Token::tokenize_str("i64 if").unwrap());
    println!("{:?}", tks.pick::<Type>());
}
