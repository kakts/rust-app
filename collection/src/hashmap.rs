use std::collections::HashMap;

fn main() {

    // hashmap生成1
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // hashmapの値取得
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("score:{:?}", score);
    // hashmap生成2
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // HashMap<_, _>という型注釈が必要
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
