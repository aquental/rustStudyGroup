mod car;

use car::{CarAdapter, ModernCarInterface, VintageCar};

fn main() {
    // TODO: Create a VintageCar object
    let vintage_car = VintageCar::new();
    // TODO: Instantiate a CarAdapter object with the VintageCar object
    let car_adapter = CarAdapter::new(vintage_car);
    // TODO: Call the start_engine, accelerate, and brake methods on the CarAdapter object
    car_adapter.start_engine();
    car_adapter.accelerate();
    car_adapter.brake();
}
