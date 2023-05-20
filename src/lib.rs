#![no_std]

#[macro_use]
extern crate alloc;

/// Lexic analyzer.
pub mod lexer;

/// Syntax analyzer.
pub mod parser;

// Reexport AST module.
mod ast;
pub use ast::*;

// Resport common types
mod common;
pub use common::*;
