/*
Update the distance_from_origin method in a Point struct to use a different distance metric.
Currently (./src/structs.rs), the method uses L2 (Euclidean) distance: SQRT(POW(x, 2) + POW(y, 2)).
Change it to use L1 (Manhattan) distance: ∣x∣+∣y∣ (sum of absolute values).
*/

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated function (constructor)
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // TODO: Change this method to calculate the Manhattan distance from origin instead of the Euclidean distance
    fn distance_from_origin(&self) -> f64 {
        (self.x.abs() + self.y.abs()) as f64
    }
}

fn main() {
    let point = Point::new(3, 4);
    println!("Distance from origin: {}", point.distance_from_origin());
}
