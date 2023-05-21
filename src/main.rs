use calf;

// All parts of syntax
const _CODE_1: &str = r#"
    num =
        x >= 0 && y >= 0
        ? x * y
        : 0
    
    num + 7 * 9
"#;

// Expression statements
const _CODE_2: &str = r#"
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
const _CODE_3: &str = r#"
        10   5 + var
        x = 10   y = (var + num) - 7
        20 + num - 8
        4.87
        num / 4 + 10 * (!!var - 2) % 3 == -num + 9 != 8 * num > !19
    "#;

fn main() {
    println!("---- CALF ----\n");

    let code = _CODE_3;
    let ast = calf::Ast::<f32>::build(code).expect("Error generating AST");
    for stmt in ast.statements {
        println!("{:?}\n", stmt);
    }
}
