/// title: destructuring
/// tags: destructuring, destructure

#[derive(Debug)]
struct Monster {
    name: String,
    age: u8,
}

fn destructuring() {
    let rawr = Monster {
        name: "rawr".to_string(),
        age: 8,
    };
    
    println!("{:#?}", rawr);
    
    let Monster {
        name,
        age,
    } = rawr;
    
    println!("{} and {}", name, age);
}