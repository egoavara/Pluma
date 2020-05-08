#![feature(const_fn)]

extern crate lazy_static;

use std::fs::File;
use crate::pluma::ir::types::Type;
use crate::pluma::token::{NomTrait, GlobalIdentifier, Identifier};
use crate::pluma::ir::define::Define;

mod pluma;

fn main() {
    let mut f = File::open("./examples/00_helloworld.pluma").unwrap();
    //
    // let ori = Token::tokenize_str("i64").unwrap();
    // let a = Type::parse(ori.as_slice(), |_|()).unwrap();
    let res = Define::parse("fn main(var args []str){}").unwrap();
    println!("{:?}", res);
}
