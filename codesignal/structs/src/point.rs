/*
Find and correct errors in ./src/point.rs that defines a Point struct. In particular, the following code contains 2 bugs that prevent it from correctly executing.
*/
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated function (constructor)
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    // Method to calculate distance from origin
    fn distance_from_origin(self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

fn main() {
    let point = Point::new(3, 4);
    println!("Distance from origin: {}", point.distance_from_origin());
}
