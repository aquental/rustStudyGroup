mod accessory_factory;
mod controller;
mod game_setup;
mod headset;

use accessory_factory::{AccessoryFactory, PlayStationFactory, XboxFactory};
use game_setup::GameSetup;

fn main() {
    let system_type = "PlayStation"; // Change this to "Xbox" to see Xbox setup
    let factory: Box<dyn AccessoryFactory> = match system_type {
        "PlayStation" => Box::new(PlayStationFactory),
        "Xbox" => Box::new(XboxFactory),
        _ => panic!("Unknown system type."),
    };

    let setup = GameSetup::new(factory);
    setup.setup();
}
