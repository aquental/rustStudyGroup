// TODO: Define a trait Shape
// - Declare an abstract method draw()
pub trait Shape {
    fn draw(&self);
}

// TODO: Define a struct Circle that implements Shape
// - Implement the draw() method to print "Drawing a Circle."
pub struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a Circle.");
    }
}

// TODO: Define a struct Square that implements Shape
// - Implement the draw() method to print "Drawing a Square."
pub struct Square;

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a Square.");
    }
}

// Rectangle struct (referenced in drawing_application.rs)
pub struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing a Rectangle.");
    }
}
