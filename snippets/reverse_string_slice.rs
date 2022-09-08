/// title: reverse string slice
/// tags: reverse

fn reverse_string_slice() {
    let name = "jotcode";
    let reverse = name.chars().rev().collect::<String>();

    println!("{}", reverse);
}
