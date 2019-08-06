/*
표준 라이브러리에 정의되 있음
enum Option<T> {
    Some<T>,
    None
}
*/

pub fn option_basic_func() {
    // Option 은 기본적으로 스코프에 포함되어, 명시적으로 로드하지 않아도 사용할 수 있음.

    let some_number = Some(12);
    let some_string = Some("a string");

    // None를 사용할 땐 타입으 명시해주어야 함. rust컴파일러가 타입을 추론할 수 없기 때문에
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", absent_number);
}