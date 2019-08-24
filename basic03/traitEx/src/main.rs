mod trait_basic;
mod trait_generic_bound;

trait CustomTrait {
    fn to_string(&self) -> String;
}

impl<T> CustomTrait for Vec<T> {
    fn to_string(&self) -> String {
        format!("{}", &self.len())
    }
}

fn main() {
    println!("============ trait_basic_func ===========");
    trait_basic::trait_basic_func();
    println!("============ trait_generic_bound ===========");
    trait_generic_bound::trait_generic_bound_func();

    println!("============ Vec impl CustomTrait ===========");
    let vec_var = vec!["하이", "하이2", "하이3"];
    println!("{}", vec_var.to_string());
}
