pub fn option_match_func() {
    let some_plus = plus_one(Some(4));
    let none_plus = plus_one(None);

    println!("some_plus: {:?}", some_plus);
    println!("none_plus: {:?}", none_plus);
}

fn plus_one(opt: Option<i32>) -> Option<i32> {
    match opt {
        // Option match로 control할시 None이거나 Some(<i32>) 둘 다 case가 있어야 함
        None => None,
        Some(number) => Some(number + 1)
    }
}