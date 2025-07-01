mod space_factory;
mod space_mission;
mod spaceship;
mod spacesuit;

use crate::space_factory::{NASAFactory, SpaceFactory, SpaceXFactory};
use crate::space_mission::SpaceMission;

fn main() {
    // TODO: Initialize a string variable `agency_type` with the value "NASA".
    let agency_type = String::from("NASA");

    // TODO: Use a match expression on `agency_type` to assign a Box<dyn SpaceFactory> to `factory`:
    // - If "NASA", use `Box::new(NASAFactory)`.
    // - If "SpaceX", use `Box::new(SpaceXFactory)`.
    // - For other values, panic with message "Unknown agency type."
    let factory: Box<dyn SpaceFactory> = match agency_type.as_str() {
        "NASA" => Box::new(NASAFactory),
        "SpaceX" => Box::new(SpaceXFactory),
        _ => panic!("Unknown agency type."),
    };

    // TODO: Create a `SpaceMission` using `factory` and call the `prepare()` method.
    let mission = SpaceMission::new(factory);
    mission.prepare();
}
