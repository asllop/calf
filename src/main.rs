use std::process::exit;

use calf::lexer::{scan_token, Lexeme, Pos};

fn main() {
    println!("CALF\n");

    let mut code = r#"
    f_acc = |n,acc| n > 1 ? f_acc(n - 1, n * acc) : acc
    fact = |n| f_acc(n, 1)
    
    fact(6) // this is a comment ^ * / &%
    
    filter([1,2,3,4], |x| x % 2 == 0)
    2 * 6 / (4 + 10)

    nums = [1,2,3,4,5]
    nums = nums * [0;5;2]
    nums = nums[0..1] # 3 # [5,8]

    exit
    "#;

    let mut tokens = vec![];
    let mut pos = Pos::new(0, 0);
    loop {
        match scan_token::<f64>(code, pos) {
            Ok((token, code_remain, curr_pos)) => {
                pos = curr_pos;
                code = code_remain;
                match token.lexeme {
                    Lexeme::Number(_) | Lexeme::Ident(_) | Lexeme::Other(_) => tokens.push(token),
                    Lexeme::EOF => break,
                    _ => {}
                }
            }
            Err(err) => {
                println!("Error = {:?}", err);
                exit(1);
            },
        }
    }

    for t in tokens {
        println!("{:?}", t);
    }
}
