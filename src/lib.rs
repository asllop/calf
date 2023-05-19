#![no_std]

#[macro_use]
extern crate alloc;

/// Lexic analyzer.
pub mod lexer;

/// Syntax analyzer.
pub mod syntaxer;

/// From code to AST.
pub mod parser;
