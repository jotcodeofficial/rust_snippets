/// title: vec with capacity
/// tags: vec, capacity

fn vec_with_capacity() {
    let mut sweets: Vec<u8> = Vec::with_capacity(5);
    sweets.push(1);
    sweets.push(2);
    sweets.push(3);

    println!("{:#?}", sweets);
}
