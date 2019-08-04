pub fn function_references_brrowing_func() {
    println!("================ borrowing_func ================");
    let str1 = String::from("hello");
    borrowing_func(&str1);
    println!("str 소유권은 나에게 있음: {}", str1);

    println!("================ borrowing_func2 ================");
    let mut str2 = String::from("hello2");
    borrowing_func2(&mut str2);
    println!("{}", str2);

    println!("================ borrowing_func3 ================");
    let mut str3 = String::from("hello3");
    borrowing_func3(&mut str3);
    borrowing_func3(&mut str3);
    println!("{}", str3);
}

fn borrowing_func(str1: &String) {
    println!("str변수를 빌렷다: {}", str1);

    // 파라미터로 넘어온 str1의 값에 영양을 주지 않음
    // let str1 = String::from("hello2");
}

fn borrowing_func2(str1: &mut String) {
    println!("str변수를 빌렷다: {}", str1);

    // str1의 주소를 변경할 수 있다.
    *str1 = String::from("borrowing_func2에서 새로운 stack과 heap이 생성됐음");
}

fn borrowing_func3(str1: &mut String) {
    str1.push_str(" : borrowing_func3에서 변경됨");
}