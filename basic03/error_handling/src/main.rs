mod panic;
mod result;

fn main() {
    println!("=============== panic_basic_func ===============");
    panic::panic_basic::panic_basic_func();
    println!("=============== result_basic_func ===============");
    result::result_basic::result_basic_func();
    println!("=============== result_spread_func ===============");
    result::result_spread::result_spread_func();
}
