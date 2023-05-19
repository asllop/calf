use calf;

fn main() {
    println!("CALF\n");

    // All parts of syntax
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

    // Expression statements
    let _code_2 = r#"
        10   5 + var
        num
        10 +// his is a comment ¿
            num
        10 + 9 -
            (
                12 + 67 - num
            )
        (var + num) - 7
    "#;

    // Expression statements and assignment statements
    let _code_3 = r#"
        10   5 + var
        x = 10   y = (var + num) - 7
        20 + num - 8
    "#;

    let code = _code_3;

    // let mut lexer = Lexer::new(code);
    // let mut tokens = vec![];

    // loop {
    //     match lexer.scan_token::<f64>() {
    //         Ok(token) => match token.lexeme {
    //             Lexeme::EOF => break,
    //             _ => tokens.push(token),
    //         },
    //         Err(err) => {
    //             println!("Error = {:?}", err);
    //             exit(1);
    //         }
    //     }
    // }

    // for t in tokens {
    //     println!("{:?}", t);
    // }

    println!("\nPARSER\n");

    // let mut parser = Parser::<f64>::new(code);
    // loop {
    //     match parser.scan_stmt() {
    //         Ok(stmt) => {
    //             if let Stmt::Expr(e) = &stmt {
    //                 if let Syntagma::Empty = e.syn {
    //                     println!("------> Empty expression at {:?}", e.pos);
    //                     break;
    //                 } else {
    //                     println!("{:?}\n", stmt);
    //                 }
    //             } else {
    //                 println!("{:?}\n", stmt);
    //             }
    //         }
    //         Err(err) => {
    //             println!("\nError = {:?}", err);
    //             exit(1);
    //         }
    //     }
    //     if parser.is_end() {
    //         println!("\nEND!");
    //         break;
    //     }
    // }

    let ast = calf::Ast::<f64>::build(code).unwrap();
    for stmt in ast.statements {
        println!("{:?}\n", stmt);
    }
}
