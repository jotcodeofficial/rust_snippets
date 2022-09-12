/// title: destructuring
/// tags: destructuring, destructure

#[derive(Debug)]
struct Monster {
    name: String,
    age: u8,
    color: String,
}

fn destructuring() {
    let rawr = Monster {
        name: "rawr".to_string(),
        age: 8,
        color: "red".to_string(),
    };
    
    println!("{:#?}", rawr);
    
    let Monster {
        name: n,
        age,
        ..
    } = rawr;
    
    println!("{} and {}", n, age);
}