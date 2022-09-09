/// title: enum basic
/// tags: enum

#[derive(Debug)]
enum Choice {
    Choice1,
    Choice2,
}

fn create_choice(amount: usize) -> Choice {
    match amount {
        5 => Choice::Choice1,
        _ => Choice::Choice2,
    }
}

fn choice_result(result: &Choice) {
    match result {
        Choice::Choice1 => println!("choice 1 is the result"),
        _ => println!("choice 2 is the result"),
    }
}

fn enum_basic() {
    let amount: usize = 5;
    let choice = create_choice(amount);
    choice_result(&choice);
}
