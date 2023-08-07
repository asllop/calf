use crate::{
    common::CalfErr,
    parser::{Parser, Stmt},
    semantic,
};
use alloc::vec::Vec;
use core::{fmt::Debug, str::FromStr};

#[derive(Debug)]
/// Abstract Syntax Tree.
pub struct Ast<T> {
    pub statements: Vec<Stmt<T>>,
}

impl<'a, T> Ast<T>
where
    T: FromStr + Debug + PartialEq,
    <T as FromStr>::Err: Debug,
{
    pub fn build(code: &'a str) -> Result<Self, CalfErr> {
        let mut ast = Self {
            statements: Default::default(),
        };
        let mut parser = Parser::new(code);
        loop {
            let stmt = parser.scan_stmt()?;
            ast.statements.push(stmt);
            if parser.ended() {
                break;
            }
        }
        semantic::check(&ast.statements)?;
        Ok(ast)
    }
}
