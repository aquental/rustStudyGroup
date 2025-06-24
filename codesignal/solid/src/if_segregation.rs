/*
Focus on the Interface Segregation Principle.
The existing code (./src/if_segregation.rs) involves a Vehicle trait that combines functionalities needed by different types of vehicles, leading to unused and unnecessary method implementations.Refactor the code by breaking down this trait into smaller, more specific traits that cater to the needs of different types of vehicles.
The current setup complicates the trait design by forcing unrelated functionalities onto different vehicle types.
Streamline the traits and ensure that each vehicle implements only the methods relevant to its functionality.

✅ Changes Made:

1. Split the monolithic Vehicle trait into three smaller, more specific traits:
•  Drivable - for vehicles that can be driven (with engines)
•  HasDoors - for vehicles that have doors
•  Pedalable - for vehicles that are pedaled (human-powered)
2. Updated Car implementation:
•  Implements Drivable trait (cars can be driven)
•  Implements HasDoors trait (cars have doors)
•  No longer forced to implement irrelevant pedal() method
3. Updated Bicycle implementation:
•  Implements only Pedalable trait (bicycles are pedaled)
•  No longer forced to implement irrelevant drive() and open_door() methods
4. Updated main function:
•  Car calls drive() and open_door() methods
•  Bicycle calls only pedal() method

✅ Benefits of this refactoring:

•  Interface Segregation: Each vehicle type now implements only the methods that are relevant to its functionality
•  No Forced Dependencies: Vehicles are no longer forced to implement methods they don't need
•  Better Maintainability: Each trait has a single, focused responsibility
•  Extensibility: Easy to add new vehicle types that implement only the traits they need
•  Type Safety: Compile-time guarantees that methods are only called on appropriate types
 */

// Split the Vehicle trait into smaller, more specific traits
trait Drivable {
    fn drive(&self);
}

trait HasDoors {
    fn open_door(&self);
}

trait Pedalable {
    fn pedal(&self);
}

struct Car;

// Implement the appropriate traits for Car
impl Drivable for Car {
    fn drive(&self) {
        println!("Car is driving");
    }
}

impl HasDoors for Car {
    fn open_door(&self) {
        println!("Car door is opening");
    }
}

struct Bicycle;

// Implement the appropriate traits for Bicycle
impl Pedalable for Bicycle {
    fn pedal(&self) {
        println!("Bicycle is pedaling");
    }
}

fn main() {
    let car = Car;
    car.drive();
    car.open_door();

    let bicycle = Bicycle;
    // Updated method calls for Bicycle based on the new traits
    bicycle.pedal();
}
