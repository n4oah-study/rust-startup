pub fn match_mod_func() {
    println!("=============== match_basic ===============");
    match_basic();
    println!("=============== match_tuple ===============");
    match_tuple();
}

// match 기본, rust에서는 C언어의 switch와 비슷한 역할임
fn match_basic() {
    let number = 3;

    println!("number: {}", number);
    match number {
        1 => println!("1"),
        2 | 3 | 5 | 7 | 11 => println!("2, 3, 5, 7, 11"),
        13...19 => println!("13~19"),
        1...12 => println!("위에 있는 조건 제외한 1~12"),
        _ => println!("그 외")
    }
}

fn match_tuple() {
    let pair = (0, -2, 1);

    println!("Tell me about {:?}", pair);
    match pair {
        (0, -2, x) => println!("0, -2, {}", x), // <-가 나옴
        (0, -2, 1) => println!("all"),
        _ => println!("그 외")
    }
}