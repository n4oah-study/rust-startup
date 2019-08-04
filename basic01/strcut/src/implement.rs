#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

impl Person {
    fn get_full_name(&self) -> String {
        format!("이름: {} {}", self.first_name, self.last_name)
    }

    fn get_age(&self) -> String {
        format!("나이: {}세", self.age)
    }
}

pub fn implement() {
    let person = Person {
        first_name: "김".to_string(),
        last_name: "호진".to_string(),
        age: 21
    };

    println!("{}", person.get_full_name());
    println!("{}", person.get_age());
}