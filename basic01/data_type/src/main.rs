fn main() {
    primitive();
}

fn primitive() {
    // =============================== primitive 타입들 =============================== //
    let a:u32 = 2;
    let b:f64 = 3.1412314122;
    let c = 'a';
    println!("u32: {} | f64: {} | char {}", a, b, c);

    // =============================== 리터럴 =============================== //

    println!("{}", 0xff); // 정수형 리터럴

    // =============================== 튜플 =============================== //
    let tuple: (u32, f64) = (12, 234.4);
    println!("{} | {}", tuple.0, tuple.1);

    // tuple 그대로 print
    println!("{:?}", tuple);

    // tuple 여러개의 변수로 바인딩할 수 있음
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);

}