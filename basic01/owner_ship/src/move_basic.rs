pub fn move_basic_func() {
    println!("========= move_func() =========");
    move_func();
    println!("========= heap_copy_func() =========");
    heap_copy_func();
}

fn move_func() {
    let str1 = String::from("hello");
    let str2 = str1;

    // println!("{}", str1); // error
    println!("{}", str2); // output: hello
}

fn heap_copy_func() {
    let str1 = String::from("hello");
    let str2 = str1.clone();

    println!("{}", str1); // output: hello
    println!("{}", str2); // output: hello
}
