/// title: ownership
/// tags: ownership

fn print_food(food_name: String) {
    println!("{}", food_name);
}

fn ownership() {
    let food = String::from("bananas");
    print_food(food); // the reference to the food has been passed to the print food function, and dies at the end of that functions scope
    print_food(food); // this will not work because the ownership has already been passed to the print function and has already died
    // the solution is to instead pass a reference to the food
}
