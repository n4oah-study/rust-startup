pub fn option_if_let_func() {
    let some_three = Some(3);
    if let Some(2) = some_three {
        println!("three ");
    }
}
