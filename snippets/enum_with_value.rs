/// title: enum with value
/// tags: enum, enums, value

#[derive(Debug)]
enum Choice {
    Choice1(String),
    Choice2(String),
}

fn create_choice(amount: usize) -> Choice {
    // import choice here so you don't need to specify Choice:: each time:
    use Choice::*;
    match amount {
        5 => Choice1(String::from("choice 1 is the result")),
        _ => Choice2(String::from("choice 2 is the result")),
    }
}

fn choice_result(result: &Choice) {
    match result {
        Choice::Choice1(value) => println!("{}", value),
        _ => println!("choice 2 is the result"),
    }
}

fn enum_with_value() {
    let amount: usize = 5;
    let choice = create_choice(amount);
    choice_result(&choice);
}
