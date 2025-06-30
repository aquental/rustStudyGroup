use crate::vehicle::{Bike, Car, Motorcycle, Vehicle};

// TODO: Create a trait named VehicleCreator with a method 'create_vehicle'.
pub trait VehicleCreator {
    fn create_vehicle(&self) -> Box<dyn Vehicle>;
}

// TODO: Create a struct named CarCreator that implements the VehicleCreator trait and provides the 'create_vehicle' method to return a new Car.
pub struct CarCreator;
impl VehicleCreator for CarCreator {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        Box::new(Car {})
    }
}

// TODO: Create a struct named BikeCreator that implements the VehicleCreator trait and provides the 'create_vehicle' method to return a new Bike.
pub struct BikeCreator;
impl VehicleCreator for BikeCreator {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        Box::new(Bike {})
    }
}

// TODO: Create a struct named MotorcycleCreator that implements the VehicleCreator trait and provides the 'create_vehicle' method to return a new Motorcycle.
pub struct MotorcycleCreator;
impl VehicleCreator for MotorcycleCreator {
    fn create_vehicle(&self) -> Box<dyn Vehicle> {
        Box::new(Motorcycle {})
    }
}
