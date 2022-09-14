/// title: generics_multiple
/// tags: generic, generics

use std::fmt::{Display, Debug};
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Robot {
    name: String,
    age: u32,
}

fn compare_and_display<T, U, V>(statement: T, num_1: U, num_2: U, robot: V)
where T: Display,
      U: Display + PartialOrd,
      V: Debug,
{
    println!("{}, {}, {}, {}, {:?}", statement, num_1, num_2, num_1 > num_2, robot);
}

fn generics_multiple() {
    let robot = Robot {
        name: "ROBO".to_string(),
        age: 5,
    };
    compare_and_display("hello", 4, 5, robot);
}