// TODO: Define the VintageCar struct
// It should have methods:
// - ignite(): prints "Igniting the vintage car engine."
// - drive(): prints "Driving the vintage car."
// - halt(): prints "Stopping the vintage car."
pub struct VintageCar;
impl VintageCar {
    pub fn ignite(&self) {
        println!("Igniting the vintage car engine.");
    }
    pub fn drive(&self) {
        println!("Driving the vintage car.");
    }
    pub fn halt(&self) {
        println!("Stopping the vintage car.");
    }
    pub fn new() -> VintageCar {
        VintageCar
    }
}

// TODO: Define the ModernCarInterface trait
// It should have methods:
// - start_engine()
// - accelerate()
// - brake()
pub trait ModernCarInterface {
    fn start_engine(&self);
    fn accelerate(&self);
    fn brake(&self);
}

// TODO: Define the CarAdapter struct that implements ModernCarInterface
// It should have a constructor that takes a VintageCar instance and initializes it.
// Implement the methods start_engine, accelerate, and brake to call the appropriate methods of VintageCar.
pub struct CarAdapter {
    pub vintage_car: VintageCar,
}

impl ModernCarInterface for CarAdapter {
    fn start_engine(&self) {
        self.vintage_car.ignite();
    }
    fn accelerate(&self) {
        self.vintage_car.drive();
    }
    fn brake(&self) {
        self.vintage_car.halt();
    }
}
