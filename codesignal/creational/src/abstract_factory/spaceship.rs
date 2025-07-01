// TODO: Define a trait `Spaceship` with a method `fn blast_off(&self);`.
pub trait Spaceship {
    fn blast_off(&self);
}

// Define NASA spaceship struct
pub struct NASASpaceship;

// TODO: Implement `NASASpaceship` as a struct that implements `Spaceship`.
// - Define the `blast_off` method to print "Blasting off with NASA spaceship."
impl Spaceship for NASASpaceship {
    fn blast_off(&self) {
        println!("Blasting off with NASA spaceship.");
    }
}

// Define SpaceX spaceship struct
pub struct SpaceXSpaceship;

// TODO: Implement `SpaceXSpaceship` as a struct that implements `Spaceship`.
// - Define the `blast_off` method to print "Blasting off with SpaceX spaceship."
impl Spaceship for SpaceXSpaceship {
    fn blast_off(&self) {
        println!("Blasting off with SpaceX spaceship.");
    }
}
