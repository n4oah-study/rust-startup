# Rust의 ownership 개념
## 메모리 Move
* 아래 코드에서 str2에 str1을 대입하면 str1은 더 이상 사용할 수 없는 변수가 됨(무효화)
    * 다른 언어에서는 얕은 복사가 되지만 rust에서는 str1이 무효화 되기 때문에 move라고 부름
    * str1의 heap영역은 메모리 해제 대상에서 제외함 (str2와 곂치기 때문에 두 번 해제할 우려가 있기 때문에)
    * 그로인해 str1을 print하려하면 컴파일 에러가 나옴
```rs
let str1 = String::from("hello");
let str2 = str1;

println!("{}", str1); // error
println!("{}", str2); // output: hello
```
* move하지 않고 힙 영역도 복사하고 싶다면 clone 메서드 사용
```rs
let str1 = String::from("hello");
let str2 = str1.clone();

println!("{}", str1); // output: hello
println!("{}", str2); // output: hello
```
* 함수(메소드)에서도 파라미터에 값을 넘기는 행위도 새로운 stack변수에 값을 대입하는 개념이라 move됨
    * function에서 parameter로 받은 값을 다시 호출한 scope에서 사용하고 싶으면 다시 return 하면 됨
        * 모든 function에 parameter로 받은 값을 넘겨주는 것은 과한 작업이 될 수 있기 때문에 참조자를 borrowing하면 됨
## references-borrowing
* function에서 references를 파라미터로 던질 때 소유권을 넘겨주는 것을(move) 원하지 않는다면 &키워드로 변수를 빌려줄 수 있음
```rs
fn main() {
    let str = String::from("hello");
    borrowing_func(&str);
    println!("str 소유권은 나에게 있음: {}", str);
}

fn borrowing_func(str: &String) {
    println!("str변수를 빌렷다: {}", str);
}
```

* _owner스코프가 종료되면 메모리 해제함_