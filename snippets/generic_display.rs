/// title: generic display
/// tags: generic, display

use std::fmt::Display;

fn test_generic<T: Display>(anything: T) {
    println!("{}", anything);
}

fn generic_display() {
    test_generic(5);
    test_generic(5.5);
    test_generic("5");
}
