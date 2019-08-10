pub fn vector_basic_func() {
    // push해주려면 mut을 써야 함
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    vec1.push(5);
    println!("vec1: {:?}", vec1);

    // 초기화 하면서 값을 넣을 수 있음
    // immutable이기 때문에 새로운 값을 push할 수 는 없음
    let vec2 = vec![6, 7, 8, 9, 10];
    // vec2.push(11); // error
    println!("vec2: {:?}", vec2);

    vector_get();
    vector_get_reference_and_push();
}

fn vector_get() {
    println!("============= vector_get ==============");
    let vector = vec![1, 2, 3, 4, 5];
    let opt: Option<&i32> = vector.get(2);
    println!("vector-opt: {:?}", opt);
    if let Some(val) = opt {
        println!("vector-opt safe value: {}", val);
    }
    println!("vector-opt opt.unwrap(): {}", opt.unwrap());

    let vector2 = vec![1, 2, 3, 4, 5];
    println!("&vector2[index]: {}", &vector2[3]);
}

fn vector_get_reference_and_push() {
    /*
    // 아래 코드는 에러가 남
    // 참고: https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable

    let mut vector = vec![1, 2, 3, 4, 5];
    let first = &vector[0];
    vector.push(6); // error: mutable borrow occurs here

    println!("first: {}", first);
    println!("{:?}", vector);
    */

    // 참조된 변수를 push하기 전에 써주기
    let mut vector = vec![1, 2, 3, 4, 5];
    let first = &vector[0];
    println!("first: {}", first);
    vector.push(6);
    println!("{:?}", vector);

    // 참조하지 않고 변수를 clone하기
    let mut vector = vec![1, 2, 3, 4, 5];
    let first = vector.get(0).cloned();
    vector.push(6);
    println!("first: {:?}", first);
    println!("{:?}", vector);
}