/// title: lifetimes match
/// tags: lifetime, lifetimes, match

fn check_colors<'life>(rgb: (u8, u8, u8)) -> &'life str {
    let result: &str = match rgb {
        (r, g, b) if (r == 250 && g == 250 && b == 250) => "white",
        (r, g, b) if (r == 0 && g == 0 && b == 0) => "black",
        _ => "a mixture color",
    };
    return result;
}

fn main() {
    let rgb: (u8, u8, u8) = (255, 255, 255);
    println!("{}", check_colors(rgb));
}
