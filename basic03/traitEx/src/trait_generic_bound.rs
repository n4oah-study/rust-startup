use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn trait_generic_bound_func() {
    let i32Data = vec![1, 5, 0, 2, -123, 1238, 12848, 128, 128, -192472];
    println!("{}", largest(&i32Data));
}