/// title: result parse number
/// tags: result, parse

use std::num::ParseIntError;

fn parse_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn result_parse_number() {
    let parse_result = parse_number("5z");
    match parse_result {
        Ok(parse_result) => println!("{}", parse_result),
        Err(error) => println!("{}", error),
    }
}