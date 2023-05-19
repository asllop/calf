use alloc::vec::Vec;
use core::{fmt::Debug, str::FromStr};
use crate::parser::{Parser, Stmt};
use crate::CalfErr;

#[derive(Debug)]
/// Abstract Syntax Tree.
pub struct Ast<T> {
    pub statements: Vec<Stmt<T>>,
}

impl<T> Ast<T>
where
    T: FromStr + Debug + PartialEq,
    <T as FromStr>::Err: Debug,
{
    pub fn build(code: &'static str) -> Result<Self, CalfErr> {
        let mut ast = Self {
            statements: Default::default(),
        };
        let mut parser = Parser::new(code);
        loop {
            let stmt = parser.scan_stmt()?;
            ast.statements.push(stmt);
            if parser.is_end() {
                break;
            }
        }
        Ok(ast)
    }
}
