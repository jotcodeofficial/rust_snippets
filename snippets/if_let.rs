/// title: if let
/// tags, if, let

fn if_let() {
    let list = vec![1, 2, 3, 4, 5, 6];
    for i in 0..list.len() {
        if let Some(item) = list.get(i) {
            println!("the current item is: {}", item);
        }
    }
}