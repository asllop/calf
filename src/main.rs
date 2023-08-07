use calf;

// Expression statements and assignment statements
const _CODE_3: &str = r#"
    10   5 + var
    x = 10   y = (var + num) - 7
    20 + num - 8
    4.87
    num / 4 + 10 * (!!var - 2) % 3 == -num + 9 != 8 * num > !19
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

// All parts of syntax
const _CODE_1: &str = r#"
    num = x >= 0 && y >= 0 ?
        x * y
        : 0
    
    x >= 0 ?
        y >= 0 ?
            x * y
            : 0
        : 0

    foo = f(x,y) x * y + 2

    foo{
        (x + 10) / y,
        bar{x + y}
    }

    bar{
        a + 2,
        f(x) x * 10
    }

    arr#10 + 2
    arr#i + 2
    arr#(x + y*10)

    foo{10}#index
"#;

fn main() {
    println!("---- CALF ----\n");

    let code = _CODE_1;
    let ast = calf::Ast::<f32>::build(code).expect("Error generating AST");
    for stmt in ast.statements {
        println!("{:#?}\n", stmt);
        println!("------------------------------------\n");
    }
}
