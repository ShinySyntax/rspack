#![feature(box_patterns)]
#![allow(dead_code)]

use once_cell::sync::Lazy;

mod runtime;
pub use runtime::*;
mod plugin;
pub use plugin::*;
pub mod utils;
pub mod visitors;

static JS_HELPERS: Lazy<Helpers> = Lazy::new(Helpers::default);

// use typemap::{Key, TypeMap};

// pub struct JsAst;

// #[derive(Debug)]
// pub struct Value(swc_ecma_ast::Program);

// impl Key for JsAst {
//   type Value = Value;
// }

// fn transofrm(mut ctx: TypeMap) {
//   // let mut map = TypeMap::new();
//   ctx.insert::<JsAst>(Value(swc_ecma_ast::Program::Module(
//     swc_ecma_ast::Module::dummy(),
//   )));
// }
