pub fn panic_basic_func() {
    let check: bool = false;
    /*if check == false {
        panic!("check value is {}", check); // 패닉!
    }*/

    /*
    NOTE:
    C언어와 같은 언어에서는 이러햔 경우 원하는 결과가 아닐지라도 메모리 값을 주려고 함(그 메모리가 Vector영역의 메모리가 아닐지라도)
    이러한 것을 버퍼 오버라이드(Buffer overread)라고 부름 (보안에 취약)
    */
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    // vector[5]; // 패닉!
}
