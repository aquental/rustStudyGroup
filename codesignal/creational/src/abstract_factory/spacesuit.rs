// TODO: Define a trait `SpaceSuit` with a method `fn wear(&self);`.
pub trait SpaceSuit {
    fn wear(&self);
}

// Define NASA space suit struct
pub struct NASASpaceSuit;

// TODO: Implement `NASASpaceSuit` as a struct that implements `SpaceSuit`.
// - Define the `wear` method to print "Wearing NASA space suit."
impl SpaceSuit for NASASpaceSuit {
    fn wear(&self) {
        println!("Wearing NASA space suit.");
    }
}

// Define SpaceX space suit struct
pub struct SpaceXSpaceSuit;

// TODO: Implement `SpaceXSpaceSuit` as a struct that implements `SpaceSuit`.
// - Define the `wear` method to print "Wearing SpaceX space suit."
impl SpaceSuit for SpaceXSpaceSuit {
    fn wear(&self) {
        println!("Wearing SpaceX space suit.");
    }
}
