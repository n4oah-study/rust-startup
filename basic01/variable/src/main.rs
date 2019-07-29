fn main() {
    let_func();
    let_mut_func();
    const_func();
    shadowing_func();
}

fn let_func() {
    let a = 1;
    println!("let_func: {}", a);

    /*
    a = 3; // error
    println!("let_func: {}", a);
    */
}

fn let_mut_func() {
    let mut a = 1;
    println!("let_mut_func: {}", a);

    a = 3;
    println!("let_mut_func: {}", a);
}

fn const_func() {
    const VARIABLE: u32 = 1;
    println!("const_func: {}", VARIABLE);
}

fn shadowing_func() {
    /*
     * 계속해서 x변수를 let으로 선언할 수 있음.
     * x의 값도 계속 bind가능
     * 
     * ** mut으로 선언된 변수는 사용하지 않음(에러는 안 나지만 워링이나옴)
     * ** mut으로 선언한 변수는 변수의 타입을 변경할 수 없음
     */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("shadowing_func x: {}", x);
    
    let x = " AA  "; // 타입 변경
    println!("shadowing_func x: {}", x);
}