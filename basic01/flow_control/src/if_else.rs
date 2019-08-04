pub fn if_else() {
    // let 구문에서 if사용하기
    // number에 들어갈 타입은 동일해야함
    let condition = false;
    let number = if condition == true {
        3
    } else {
        5
    };

    println!("let 구문에서 if사용하기 {}일 경우: {}", condition, number);
}