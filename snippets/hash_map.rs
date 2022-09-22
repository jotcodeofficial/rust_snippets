/// title: hash map
/// tags: hash, map

use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    friends: HashMap<String, u8>,
}

fn hash_map() {
    let mut person_1 = Person {
        name: "Bob".to_string(),
        friends: HashMap::new(),
    };
    
    person_1.friends.insert("Sam".to_string(), 9);
    
    println!("{:#?}", person_1);
}