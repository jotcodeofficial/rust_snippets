/// title: tuples
/// tags: tuples

fn tuples() {
    let (a, b, c) = ("apple", "banana", "peach");
    println!("a: {}\nb: {}\nc: {}", a, b, c);

    // here we ignore the orange fruit with the _ char
    let fruit_vec = vec!["grapefruit", "watermelon", "dragonfruit", "orange"];
    let (d, e, f, _) = (fruit_vec[0], fruit_vec[1], fruit_vec[2], fruit_vec[3]);
    println!("d: {}\ne: {}\nf: {}", d, e, f);
}
