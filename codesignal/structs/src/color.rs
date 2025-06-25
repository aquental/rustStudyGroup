enum Color {
    Red,
    Green,
    Blue,
}

fn process_color(color: Color) {
    // TODO: Implement pattern matching for each enum variant
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    let color1 = Color::Red;
    let color2 = Color::Green;

    process_color(color1);
    process_color(color2);
}
