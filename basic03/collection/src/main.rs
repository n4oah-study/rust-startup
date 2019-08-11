mod vector_basic;
mod vector_enum;
mod string_basic;
mod hash_map_basic;

fn main() {
    println!("============== vector_basic_func ================");
    vector_basic::vector_basic_func();
    println!("============== vector_enum_func ================");
    vector_enum::vector_enum_func();
    println!("============== string_basic_func ================");
    string_basic::string_basic_func();
    println!("============== hash_map_basic_func ================");
    hash_map_basic::hash_map_basic_func();
}
