use std::collections::HashMap;

pub fn test_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    scores.insert(String::from("Green"), 25);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    println!("field_name: {}", field_name);

    scores.insert(String::from("Blue"), 100);
    println!("scores: {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Red")).or_insert(200);
    println!("scores: {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // let count = map.entry(word).or_insert(0);
        // *count += 1;
        *map.entry(word).or_insert(0) += 1;
    }
}
