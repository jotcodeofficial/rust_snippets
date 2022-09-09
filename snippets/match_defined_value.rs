fn check_amount(amount: usize) {
    match amount {
        rich @ 1000 => println!("You have {}, which means you are rich!", rich),
        500 => println!("You have 500"),
        _ => println!("You have an unspecified amount of money"),
    }
}

fn match_defined_value() {
    check_amount(1000);
}
