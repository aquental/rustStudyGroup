/*
Enhance the Rectangle (./src/rectangle.rs) struct by adding a method to calculate its area.
This involves writing an area method that multiplies and returns the width and height of the rectangle.
*/
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    // Method to calculate the area of the rectangle
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(5, 10);
    println!("The area of the rectangle is: {}", rect.area());
}
