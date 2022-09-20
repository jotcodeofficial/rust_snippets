/// title: while let
/// tags: while, let

fn while_let() {
    let  list = vec![
        vec!["peter", "1", "2", "3"],
        vec!["john", "4", "5", "6"],
    ];
    
    for mut sub_list in list {
        println!("the name is: {}", sub_list[0]);
        while let Some(item) = sub_list.pop() {
            if let Ok(result) = item.parse::<i32>(){
                println!("the number is: {}", result);
            }
        }
    }
}