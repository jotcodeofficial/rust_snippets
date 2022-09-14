/// title: option vec unwrap or
/// tags: option, vec, unwrap, or

fn grab_last_if_bigger_than_3(list: Vec<i32>) -> Option<Vec<i32>>{
    if list.len() > 3 {
        Some(list)
    } else {
        None
    }
}

fn option_vec_unwrap_or() {
    let list_1 = vec![1, 2, 3];
    let list_2 = vec![1, 2, 3, 4, 5, 6];
    
    let list_of_lists = vec![list_1, list_2];
    for list in list_of_lists {
         let result = grab_last_if_bigger_than_3(list);
         println!("The result is: {:?}", result.unwrap_or(vec![]));
    }
}
