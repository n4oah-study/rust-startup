pub fn function_move_basic_func() {
    let str1 = String::from("hello~~~~");
    move_func1(str1);
    // println!("function_move_basic_func: {}", str1); // error

    // 반환 값으로 str1을 다시 정의할 수 있음.
    let str1 = String::from("hello~~~~");
    let str1 = return_func1(str1);
    println!("function_move_basic_func: {}", str1);
}

fn move_func1(a: String) {
    println!("move_func1: {}", a);
}

fn return_func1(a: String) -> String {
    println!("return_func1: {}", a);
    a
}