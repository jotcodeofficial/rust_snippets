/// title: hash map if let match
/// tags: hash, map, if, let, match

use std::collections::HashMap;

fn hash_map_if_let_match() {
    let numbers = vec!["One", "Two", "Three"];
    
    let mut list = HashMap::new();
    
    let mut index = 5;
    for num in numbers {
        list.insert(num, index);
        index += 1;
    }
    
    // if some(5) is the same as the value from the key of "One"
    if let Some(5) = list.get("One") {
        println!("Hey let some");
    }
    
    // or you could do a match
    match list.get("One") {
        Some(5) => println!("Hey match"),
        None => println!("None"),
        _ => println!("wtf"),
    }

}