# rust의 변수
## 변수의 종류
* let: 기본적으로 불변성을 갖고있음
 * 가변성 변수 선언방법
 <pre>
let mut a = 1;
println!("{}", a) // output 1
a = 2;
println!("{}", a) // output 2
</pre>

* const: 상수 (mut을 쓸 수 없음)
* let과 const의 차이
 * const는 함수 호출의 결과값이나 그 외에 런타임시에 결정되는 값이 설정될 수는 없음
## 변수 shadowing
<pre>
let x = 5;
let x = x + 1;
let x = x * 2;
println!("{}", x); 
</pre>

## 가변성, 불변성
* rust의 기본 변수는 불변성
 * Rust가 제공하는 안전성과 손쉬운 동시성이라는 장점을 취할 수 있도록 코드를 작성하게끔 강제하는 요소 중 하나
 * _NOTE: 기본 변수가 불변성이 되기 때문에 컴파일시 최적화(?), 모든 변수를 굳이 메모리 값이 변하게 설계할 필요가 사라짐_
