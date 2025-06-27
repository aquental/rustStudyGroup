/*
The print_area function (src/print_area.rs) uses dynamic dispatch with a trait object.
Modify it to use static dispatch instead by employing generics and trait bounds.
*/

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.1416 * self.radius.powi(2)
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// TODO: Change the print_area function to use static dispatch with generics
fn print_area<T: Shape>(shape: &T) {
    println!("The area is {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    print_area(&circle);
    print_area(&rectangle);
}
