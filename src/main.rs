use calf::lexer::{Lexeme, Lexer};
use calf::parser::{Parser, Stmt, SynUnit};
use std::process::exit;

fn main() {
    println!("CALF\n");

    let _code_1 = r#" // CALF example
        f_acc = |n,acc| n > 1 ? f_acc(n - 1, n * acc) : acc
        fact = |n| f_acc(n, 1)
        
        fact(6) // this is a comment ^ * / &%
        
        filter([1,2,3,4], |x| x % 2 == 0)
        2 * 6 / (4 + 10)

        nums = [1,2,3,4,5]
        nums = nums * [0;5;2]
        nums = nums[0..1] # 3 # [5,8]
    "#;
    
    let _code_2 = r#"
        10
        num
        10 + // his is a comment ¿
            num
        10 + 9 -
            (
                12 + 67 - num
            )
    "#;

    let code = _code_2;

    let mut lexer = Lexer::new(code);
    let mut tokens = vec![];

    loop {
        match lexer.scan_token::<f64>() {
            Ok(token) => match token.lexeme {
                Lexeme::EOF => break,
                _ => tokens.push(token)
            },
            Err(err) => {
                println!("Error = {:?}", err);
                exit(1);
            }
        }
    }

    for t in tokens {
        println!("{:?}", t);
    }

    println!("\nPARSER\n");

    let mut parser = Parser::<f64>::new(code);
    loop {
        match parser.scan_stmt() {
            Ok(stmt) => {
                if let Stmt::Expr(e) = &stmt {
                    if let SynUnit::Empty = e.syn_unit {
                        println!("------> Empty expression at {:?}", e.pos);
                    }
                    else {
                        println!("{:?}\n", stmt);
                    }
                }
                else {
                    println!("{:?}\n", stmt);
                }
            }
            Err(err) => {
                println!("\nError = {:?}", err);
                exit(1);
            }
        }
        if parser.is_end() {
            println!("\nEND!");
            break;
        }
    }
}
