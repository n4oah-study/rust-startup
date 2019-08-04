pub mod loop_func {
    pub fn func() {
        let mut number = 0;
        loop {
            if number == 5 {
                break;
            }

            // 후위연산자가 없으니 굉장이 이상해진다
            println!("loop: {}", {
                number += 1;
                number -1
            });
        }
    }
}

pub mod while_func {
    pub fn func() {
        let mut number = 0;

        while number != 5 {
            println!("while: {}", number);

            number = number + 1;
        }
    }
}

pub mod for_func {
    pub fn func() {
        let array = [1, 4, 2, 6, 7, 10];
        
        // 배열의 순서대로 element를 뽑음
        for element in array.iter() {
            println!("for iter: {}", element);
        }

        // element와 index를 같이 뽑음
        for (idx, element) in array.iter().enumerate() {
            println!("for iter enumerate: {} - {}", idx, element);
        }

        // 0 ~ 5
        for number in 0..5 {
            println!("for 0..5 count: {}", number);
        }

        // 0 ~ 5 rev
        for number in (0..5).rev() {
            println!("for rev(): {}", number);
        }
    }
}