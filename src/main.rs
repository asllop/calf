use calf::lexer::scan;

fn main() {
    println!("CALF\n");

    scan(r#"
        f_acc = |n,acc| n > 1 ? f_acc(n - 1, n * acc) : acc
        fact = |n| f_acc(n, 1)
        
        fact(6)

        filter([1,2,3,4], |x| x % 2 == 0)
    "#).unwrap();
}
