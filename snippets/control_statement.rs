/// title: control statement
/// tags: control, if, else

fn control_statement() {
    let amount: usize = 100;
    let result: &str;

    if amount > 100 {
        result = "too high";
    } else if amount < 100 {
        result = "too low";
    } else if amount == 100 {
        result = "perfect";
    } else {
        result = "invalid amount";
    }

    println!("{}", result);
}
