mod vehicle;
mod vehicle_creator;

use vehicle_creator::{BikeCreator, CarCreator, MotorcycleCreator, VehicleCreator};

fn main() {
    // TODO: Create a Car using CarCreator and call its drive method.
    let car_creator = CarCreator;
    let c = car_creator.create_vehicle();
    c.drive();

    // TODO: Create a Bike using BikeCreator and call its drive method.
    let bike_creator = BikeCreator;
    let b = bike_creator.create_vehicle();
    b.drive();

    let motorcycle_creator = MotorcycleCreator;
    let m = motorcycle_creator.create_vehicle();
    m.drive();
}
