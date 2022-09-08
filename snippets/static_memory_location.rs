/// title: static memory location
/// tags: static, memory location

fn static_memory_location() {
    // this ensures the memory location does not change
    static MY_ARRAY: [&str; 4] = ["first", "second", "third", "fourth"];

    println!("{:#?}", MY_ARRAY);
}
