/*
Implement the std::fmt::Display trait for Vehicle.
*/
use std::fmt;

struct Vehicle {
    make: String,
    model: String,
    year: u32,
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.year, self.make, self.model)
    }
}

fn main() {
    let vehicle = Vehicle {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2020,
    };

    println!("{}", vehicle);
}
