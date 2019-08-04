fn main() {
    hello_function(12, 45);
    expression_function();
    println!("expression_return: {}", expression_return(3, 5))
}

fn hello_function(a: i32, b: i32) {
    println!("a: {}, b: {}", a, b);
}

// 구문과 표현식
fn expression_function() {
    let b = 2;

    let a = {
        b + 2
    };

    println!("{} | {}", a, b);
}

fn expression_return(mut a: i32, mut b: i32) -> i32 {
    a = a + 2;
    b = b + 5;
    a + b
}