mod option_basic;
mod option_match;
mod option_if_let;

fn main() {
    println!("============= option_basic_func ================");
    option_basic::option_basic_func();
    println!("============= option_match_func ================");
    option_match::option_match_func();
    println!("============= if_let_func ================");
    option_if_let::option_if_let_func();
}
