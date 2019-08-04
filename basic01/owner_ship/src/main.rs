mod move_basic;
mod function_move_basic;
mod function_references_brrowing;

fn main() {
    println!("============= move_basic_func =============");
    move_basic::move_basic_func();
    println!("============= function_move_basic_func =============");
    function_move_basic::function_move_basic_func();
    println!("============= function_references_brrowing_func =============");
    function_references_brrowing::function_references_brrowing_func();
}
