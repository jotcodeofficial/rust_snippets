/// title: enum is positive 
/// tags: enum, positive, check
use Number::*;

enum Number {
    I32(i32),
    U32(u32),
}

fn get_number(number: i32) -> Number {
    match number.is_positive() {
        true => U32(number as u32),
        false => I32(number),
    }
}

fn enum_is_positive() {
    let my_numbers = vec![get_number(-200), get_number(300)];

    for num in my_numbers {
        match num {
            I32(number) => println!("the negative number is {}", number),
            U32(number) => println!("the positive number is {}", number),
        }
    }
}
