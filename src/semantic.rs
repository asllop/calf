use crate::{common::CalfErr, parser::Stmt};
use alloc::string::String;
use hashbrown::HashMap;

struct Symbol {
    stype: SymbolType,
    //TODO: other necessary stuff
}

enum SymbolType {
    Function,
    Variable,
}

pub fn check<T>(_statements: &[Stmt<T>]) -> Result<(), CalfErr> {
    let _symbols: HashMap<String, Symbol>;
    //TODO: check no lambda contains another lambda
    //TODO: check no named funcion captures extern variables
    //TODO: check symbol usage, don't use undefined variables
    //TODO: check variable types, don't use as a function something that is data
    Ok(())
}