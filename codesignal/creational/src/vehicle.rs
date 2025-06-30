// TODO: Create a trait named Vehicle with a method 'drive'.
pub trait Vehicle {
    fn drive(&self);
}

// TODO: Create a struct named Car that implements the Vehicle trait and provides the 'drive' method to output "Driving a Car!".
pub struct Car;
impl Vehicle for Car {
    fn drive(&self) {
        println!("Driving a Car!");
    }
}

// TODO: Create a struct named Motorcycle that implements the Vehicle trait and provides the 'drive' method to output "Riding a Motorcycle!".
pub struct Motorcycle;
impl Vehicle for Motorcycle {
    fn drive(&self) {
        println!("Riding a Motorcycle!");
    }
}

// TODO: Create a struct named Bike that implements the Vehicle trait and provides the 'drive' method to output "Riding a Bike!".
pub struct Bike;
impl Vehicle for Bike {
    fn drive(&self) {
        println!("Riding a Bike!");
    }
}
