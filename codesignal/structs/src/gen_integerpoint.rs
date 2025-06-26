struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(5.5, 10.2);
    let string_point = Point::new(String::from("hello"), String::from("world"));

    println!(
        "Integer Point: ({}, {})",
        integer_point.x(),
        integer_point.y()
    );
    println!("Float Point: ({}, {})", float_point.x(), float_point.y());
    println!("String Point: ({}, {})", string_point.x(), string_point.y());
}
