/// title: struct implements
/// tags: struct, impl, implement, implements
struct House {
    number: u32,
}

impl House {
    fn get_number(&self) -> u32 {
        self.number
    }
    
    fn change_number(&mut self, new_number: u32) {
        self.number = new_number;
    }

    fn new(number: u32) -> Self {
        Self {
            number,
        }
    }
}

fn struct_implements() {
    let mut my_house = House {
        number: 8,
    };
    
    println!("The number of my house is: {}", my_house.get_number());
    my_house.change_number(10);
    println!("The number of my house is: {}", my_house.get_number());
    
    let my_house_2 = House::new(12);
    println!("The number of my house is: {}", my_house_2.get_number());}