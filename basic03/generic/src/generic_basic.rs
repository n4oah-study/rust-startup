#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U
}

pub fn generic_basic_func() {
    let i32Data = vec![1, 5, 0, 2, -123, 1238, 12848, 128, 128, -192472];
    // println!("{}", largest(&i32Data));

    let point: Point<i32> = Point {
        x: 1,
        y: 2
    };
    println!("{:?}", point);

    let point: Point2<i32, String> = Point2 {
        x: 123,
        y: String::from("안녕하세요")
    };
    println!("{:?}", point);
}

// 가장 큰 수 or 가장 
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest { // 모든 타입이 비교연산이 되는게 아니기 때문에 error가 뜸 std::cmp::PartialOrd로 연산이 가능하도록 할 수 있음
            largest = item;
        }
    }

    largest
}
*/

/*
fn largest_partial_ord<T>(list: &[T]) -> T {

}
*/