#![no_std]

use alloc::string::String;

#[macro_use]
extern crate alloc;

/// Lexic analyzer.
pub mod lexer;

/// Syntax analyzer.
pub mod parser;

// Reexport AST module.
mod ast;
pub use ast::*;

#[derive(Debug)]
/// Compiler error.
pub struct CalfErr {
    /// Error message.
    pub message: String,
    /// Position where the error was found.
    pub pos: Pos,
}

#[derive(Debug, Default, Clone)]
/// Position of language element in the code.
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
