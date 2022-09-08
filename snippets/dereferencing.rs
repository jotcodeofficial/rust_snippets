/// title: dereferencing
/// tags: dereferencing

fn dereferencing() {
    let mut my_num = 25;
    let my_num_ref = &mut my_num;
    *my_num_ref = 50;

    println!("{}", my_num);
}
