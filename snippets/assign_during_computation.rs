/// title: assign during computation
/// tags: assign, 

fn yes() -> bool {
    true
}

fn assign_during_computation() {
    let result = if yes() {
        "yes"
    } else {
        "no"
    };
    
    println!("{}", result);
}

