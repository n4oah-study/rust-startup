mod struct_basic;
mod struct_tuple;
mod implement;

fn main() {
    println!("================= struct_func =================");
    struct_basic::struct_func();
    println!("================= struct_tuple_func =================");
    struct_tuple::struct_tuple_func();
    println!("================= implement =================");
    implement::implement();
}