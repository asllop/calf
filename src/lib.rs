#![no_std]

#[macro_use]
extern crate alloc;

mod common;
mod lexer;
mod parser;
mod semantic;

// Reexport AST module.
mod ast;
pub use ast::*;
