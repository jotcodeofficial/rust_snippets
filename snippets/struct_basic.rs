/// title: struct basic
/// tags: struct

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

fn struct_basic() {
    let age = 50;

    let person = Person {
        name: String::from("Peter"),
        age,
    };

    println!(
        "my name is {} and I am {} years old",
        person.name, person.age
    );
    println!("entire struct: \n{:?}", person);
}
