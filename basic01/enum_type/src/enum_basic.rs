#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum Fruit {
    Apple(String),
    Watermelon(String)
}

impl Fruit {
    fn my_call(&self) {
        println!("{:?} 입니다.", self)
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

pub fn enum_basic_func() {
    println!("============ Fruit ============");
    let apple = Fruit::Apple(String::from("사과"));
    let watermelon = Fruit::Watermelon(String::from("수박"));
    println!("{:?}, {:?}", apple, watermelon);

    apple.my_call();

    println!("============ IpAddr struct ============");
    let v4_addr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let v6_addr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("{:?}, {:?}", v4_addr, v6_addr);
}