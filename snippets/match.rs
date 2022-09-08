/// title: match
/// tags: match

fn match_snippet() {
    let valid = true;
    let amount = 3;

    let result = match (valid, amount) {
        (valid, amount) if valid && amount == 3 => "valid",
        _ => "invalid",
    };

    println!("{}", result);
}
