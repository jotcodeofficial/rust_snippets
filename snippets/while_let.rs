/// title: while let
/// tags: while, let

fn while_let() {
    let list = vec![
        vec!["john", "2"],
        vec!["peter", "5"],
    ];
    
    for mut sub_list in list {
        println!("the name is: {}", sub_list[0]);
        while let Some(info) = sub_list.pop() {
            if let Ok(number) = info.parse::<i32>(){
                println!("the number is: {}", number);
            }
        }
    }
}