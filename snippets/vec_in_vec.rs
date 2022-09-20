/// title: vec in vec
/// tags: vec

fn vec_in_vec() {
    let list = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    
    println!("{}", list[1][0]);
}