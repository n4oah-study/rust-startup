use std::collections::HashMap;

pub fn hash_map_basic_func() {
    let mut hash_map = HashMap::new();
    hash_map.insert("red", "빨강");
    hash_map.insert("blue", "파랑");
    hash_map.insert("blue", "파랑투"); // 덮어씌어짐
    hash_map.entry("blue").or_insert("파랑쓰리"); // 비어있을 때만 갚 덮어쓰기
    println!("hash_map: {:?}", hash_map);

    //=================== vector iterator key value ===================//
    let teams  = vec![String::from("Blue"), String::from("Yellow")]; // keys
    let initial_scores = vec![10, 50]; // values
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", teams.iter().zip(initial_scores.iter()));
    println!("{:?}", scores);
    println!("scores.get(\"Blue\"): {}", scores.get(&"Blue".to_string()).unwrap());

    //================= hash_map foreach ==================//
    for (key, value) in &scores {
        println!("key:value={}: {}", key, value);
    }
}