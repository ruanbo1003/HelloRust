#![allow(dead_code)]

use std::collections::HashMap;

pub fn tests() {
    create_new_hash_map();

}

fn create_new_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Green"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("scores:{:?}", scores);  // {"Green": 10, "Yellow": 20}

    {
        let team_name = String::from("Green");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("score:{}", score);  // 10
    }

    // access values
    {
        let team_name = String::from("NotExists");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("score:{}", score);  // 0
    }

    // iterator values
    {
        for (key, value) in scores {
            println!("{} - {}", key, value);
        }
    }
}