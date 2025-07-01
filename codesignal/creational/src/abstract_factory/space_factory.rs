use crate::spaceship::{NASASpaceship, SpaceXSpaceship, Spaceship};
use crate::spacesuit::{NASASpaceSuit, SpaceSuit, SpaceXSpaceSuit};

// TODO: Define a trait `SpaceFactory` with methods:
// - `fn create_spaceship(&self) -> Box<dyn Spaceship>;`
// - `fn create_space_suit(&self) -> Box<dyn SpaceSuit>;`
pub trait SpaceFactory {
    fn create_spaceship(&self) -> Box<dyn Spaceship>;
    fn create_space_suit(&self) -> Box<dyn SpaceSuit>;
}

// TODO: Implement `NASAFactory` struct that implements `SpaceFactory`:
// - Implement `create_spaceship` to return `Box::new(NASASpaceship)`.
// - Implement `create_space_suit` to return `Box::new(NASASpaceSuit)`.
pub struct NASAFactory;
impl SpaceFactory for NASAFactory {
    fn create_spaceship(&self) -> Box<dyn Spaceship> {
        Box::new(NASASpaceship)
    }
    fn create_space_suit(&self) -> Box<dyn SpaceSuit> {
        Box::new(NASASpaceSuit)
    }
}

// TODO: Implement `SpaceXFactory` struct that implements `SpaceFactory`:
// - Implement `create_spaceship` to return `Box::new(SpaceXSpaceship)`.
// - Implement `create_space_suit` to return `Box::new(SpaceXSpaceSuit)`.
pub struct SpaceXFactory;
impl SpaceFactory for SpaceXFactory {
    fn create_spaceship(&self) -> Box<dyn Spaceship> {
        Box::new(SpaceXSpaceship)
    }
    fn create_space_suit(&self) -> Box<dyn SpaceSuit> {
        Box::new(SpaceXSpaceSuit)
    }
}
