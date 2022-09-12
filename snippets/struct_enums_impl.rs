///title: struct enums impl
/// tags: struct, structs, enum, enums impl,

#[derive(Debug)]
struct Hero {
    power_type: PowerType,
}

#[derive(Debug)]
enum PowerType {
    Fire,
    Water,
}

impl Hero {
    fn new() -> Self {
        Self {
            power_type: PowerType::Water,
        }
    }

    fn change_to_fire(&mut self) {
        self.power_type = PowerType::Fire;
    }
}

fn struct_enum_impl() {
    let mut hero = Hero::new();
    println!("{:#?}", hero);
    hero.change_to_fire();
    println!("{:#?}", hero.power_type);
}
