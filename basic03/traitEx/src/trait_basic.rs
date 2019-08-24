trait Organism {
    fn run(&self) -> String;
    fn get_age(&self) -> i32;
    fn common_func(&self) -> String {
        format!("나의 이름은 이다.")
    }
}

struct Person {
    name: String,
    age: i32
}

impl Organism for Person {
    fn run(&self) -> String {
        format!("인간은 {}은 두 발로 달린다.", self.name)
    }
    fn get_age(&self) -> i32 {
        self.age
    }
}

struct Dog {
    name: String,
    age: i32
}

impl Organism for Dog {
    fn run(&self) -> String {
        format!("강아지인 {}은 네 발로 달린다.", &self.name)
    }
    fn get_age(&self) -> i32 {
        24 + (self.age-2) * 4
    }
}

pub fn trait_basic_func() {
    let person: Person = Person {
        name: String::from("김호진"),
        age: 21
    };
    println!("{}", person.run());
    println!("인간 {}는 인간나이로 {}살 이다.", person.name, person.get_age());
    println!("{}", person.common_func());

    let dog: Dog = Dog {
        name: String::from("꾸리"),
        age: 9
    };
    println!("{}", dog.run());
    println!("강아지 {}는 인간나이로 {}살 이다.", dog.name, dog.get_age());
    println!("{}", dog.common_func());
}