use crate::pluma::token::{GlobalIdentifier, Identifier};
use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef};
use nom::lib::std::collections::{HashMap, HashSet};
use std::any::Any;
use std::hash::{Hash, Hasher};

pub mod obj;
pub mod library;

pub struct Runtime {
    llctx: LLVMContextRef,
    llmod: LLVMModuleRef,
    refers: HashSet<RtObj>,
}

pub struct RtObj {
    ident: RtSpace,
    obj: Box<dyn Any>,
}

impl Hash for RtObj { fn hash<H: Hasher>(&self, state: &mut H) { self.ident.hash(state) } }

#[derive(Hash)]
pub enum RtSpace {
    System(Identifier),
    User(GlobalIdentifier),
    Temporary(usize),
}

