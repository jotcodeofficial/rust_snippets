/// title: enum numbers
/// tags: enum, enums

// the default numbers are 0, 1, 2, 3
// you can convert them to a number to see this
enum Items {
    Phone = 5,
    Desk = 16,
    Couch = 31,
    // a new one here without a defined number would be 32
}

fn enum_numbers() {
    use Items::*;
    let item_vec = vec![Phone, Desk, Couch];
    
    for item in item_vec {
        match item as u32 {
            cost if cost < 5 => println!("The price is too low"),
            cost if cost == 5 => println!("The price for the phone is: {}", cost),
            cost if cost == 16 => println!("The price for the desk is: {}", cost),
            cost if cost > 16 => println!("The price for the couch is: {}", cost),
            _ => println!("unknown price"),
        }
    }
}