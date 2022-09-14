/// title: generics
/// tags: generic, generics

fn generics() {
    let result = add(5);
    println!("{}", result);
}

fn add<T>(number: T) -> T {
    number
}

