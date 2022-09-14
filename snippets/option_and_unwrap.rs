/// title: option and unwrap
/// tags: option, unwrap


fn grab_last_if_bigger_than_3(list: Vec<i32>) -> Option<i32>{
    if list.len() > 3 {
        Some(list[list.len() - 1])
    } else {
        None
    }
}

fn option_and_unwrap() {
    let list_1 = vec![1, 2, 3];
    let list_2 = vec![1, 2, 3, 4, 5, 6];
    
    let result_1 = grab_last_if_bigger_than_3(list_1);
    let result_2 = grab_last_if_bigger_than_3(list_2);
    
    println!("{:?}", result_1);
    println!("{:?}", result_2.unwrap())
    
}
