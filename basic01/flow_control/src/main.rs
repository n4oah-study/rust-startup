mod if_else;
mod loop_for_while;
mod match_mod;

fn main() {
    println!("============== if_else ==============");
    if_else::if_else();
    println!("============== loop_func ==============");
    loop_for_while::loop_func::func();
    println!("============== for_func ==============");
    loop_for_while::for_func::func();
    println!("============== while_func ==============");
    loop_for_while::while_func::func();
    println!("============== match_mod_func ==============");
    match_mod::match_mod_func();
    
}
