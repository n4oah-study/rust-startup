# Rust의 에러 처리
## 특징
* Rust는 복구 가능한 에러와 복구 불가능한 에러 두 가지 범주로 묶음
* 복구 가능한 에러 Result<T, E>
* 복구 불가능한 에러 panic! 호출
## panic! 복구 불가능한 에러
* 프로그램이 중지됨
* 기본적으로, panic!이 발생하면, 프로그램은 되감기(unwinding)를 하면서 러스트가 `panic`을 마주친 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 제거 후 프로그램을 종료시킴
* panic이 발생하였을 때 스택을 거꾸로 훑어가면서 데이터를 제거하기에는 일이 많으니, 즉시 `그만두기`(abort)를 사용할 수 있음 (프로그램이 종료 된 후 os에서 메모리를 해제함)
    * Cargo.toml에 추가 (릴리즈 모드에 적용)
    ```
    [profile.release]
    panic = 'abort'
    ```
* RUST_BACKTRACE
    * RUST_BACKTRACE 환경 변수를 설정하면 에러의 원인이 된 것이 무엇인지 정확하게 백트레이스 할 수 있음
    * 백트레이스 (backtrace) 란 어떤 지점에 도달하기까지 호출해온 모든 함수의 리스트
    ```
    RUST_BACKTRACE=1 cargo run
    으로 명령어에서 환경변수 설정 가능
    ```
## Result 복구 가능한 에러
* Result는 enum타입으로 선언되어 있음
```rs
// T는 성공한 경우에 Ok Variant내에 반환할 값의 타입
// E는 실패한 경우에 Err Variant내에 반환할 에러 타입
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
## 에러 전파하기 (throw error)
* 실패할지도 모르는 함수를 구현할 때 함수 내에서 에러를 처리하는 대신 함수롤 호출하는 쪽으로 에러를 전파할 수 있음
    * Exception의 throw Exception과 비슷한 개념
