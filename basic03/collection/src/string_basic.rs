pub fn string_basic_func() {
    //======================== string push ========================//
    let mut string1: String = String::from("안녕");
    string1.push('하'); // char push
    string1.push_str("세요.");
    println!("string1.push, push_str: {}", string1);

    //======================== string add string ========================//
    let add_string1: String = String::from("안녕하");
    let add_string2: String = String::from("세요.");

    // add_string1는 move되어 새로운 string을 만듬
    // string + 연산자는 내부적으로 add function을 사용함
    // 
    let result_string = add_string1 + &add_string2;
    println!("string1(move) add string2: {}", result_string);
    //======================== string indexing ========================//
    let string_splice_from_byte = &result_string[0..3];
    println!("result_string[0..3]: {}", string_splice_from_byte); // output: 안
    println!("result_string.len(): {}", result_string.len()); // output: 3+3+3+3+3+1=16

    // rust는 유니코드를 위한 가변적 길이 인코딩인 UTF-8이 기본 인코딩임.
    // 문자의 길이가 가변적이기 때문에 문자를 보지 않고는 n번째 문자의 메모리 위치를 알 수가 없음
    // result_string을 char iterator 객체로 return, element의 2번째에 해당하는 값을 Option::Some변수로 return
    // Some안에 있는 값을 unwrap으로 value를 return함
    println!("result_string index 2: {}", result_string.chars().nth(2).unwrap());
    //======================== string bytes ========================//
    for c in "안녕하세요".chars() {
        println!("{}", c);
    }
    for b in "안녕하세요".bytes() {
        println!("{}", b); // ASCII CODE
    }
}