/// title: btree map
/// tags: btree, map

/// title: hash map
/// tags: hash, map

use std::collections::BTreeMap;

#[derive(Debug)]
struct Person {
    name: String,
    friends: BTreeMap<String, u8>,
}

fn betree_map() {
    let mut person_1 = Person {
        name: "Bob".to_string(),
        friends: BTreeMap::new(),
    };
    
    
    person_1.friends.insert("Sam".to_string(), 9);
    person_1.friends.insert("Peter".to_string(), 2);
    
    println!("{:#?}", person_1);
    
    for (name, rating) in person_1.friends {
        println!("name: {} \nrating: {}", name, rating);
    }
}