use calf::lexer::scan;

fn main() {
    println!("CALF\n");

    let tokens = scan::<f64>(
        r#"
        f_acc = |n,acc| n > 1 ? f_acc(n - 1, n * acc) : acc
        fact = |n| f_acc(n, 1)
        
        fact(6) // this is a comment ^ * / &%
        
        filter([1,2,3,4], |x| x % 2 == 0)
        2 * 6 / (4 + 10)

        nums = [1,2,3,4,5]
        nums = nums * [0;5;2]
        nums = nums[0..1] # 3 # [5,8]
    "#,
    )
    .unwrap();

    for t in tokens {
        println!("{:?}", t);
    }
}
