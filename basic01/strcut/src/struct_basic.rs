struct Person {
    name: String,
    email: String,
    age: u8,
    active: bool
}

pub fn struct_func() {
    let person1 = Person {
        name: "n4oah".to_string(),
        email: "n4oahdev@gmail.com".to_string(),
        age: 21,
        active: true
    };
    
    println!("name: {}", person1.name);
    println!("email: {}", person1.email);
    println!("age: {}", person1.age);
    println!("active: {}", person1.active);

    println!("=========================== person2 ============================");

    let person2 = build_pserson("n4oah2".to_string(), "n4oah@naver.com".to_string());
    println!("name: {}", person2.name);
    println!("email: {}", person2.email);
    println!("age: {}", person2.age);
    println!("active: {}", person2.active);

    println!("=========================== person3를 person2에서 데이터 참조하기 ============================");

    let person3 = Person {
        name: "n4aoh3".to_string(),
        ..person2
    };

    println!("name: {}", person3.name);
    println!("email: {}", person3.email);
    println!("age: {}", person3.age);
    println!("active: {}", person3.active);
}

// 축약법
fn build_pserson(name: String, email: String) -> Person {
    Person {
        name,
        email,
        age: 22,
        active: false
    }
}