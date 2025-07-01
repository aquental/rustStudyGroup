mod house;
mod house_builder;

use house_builder::{HouseBuilder, WoodenHouseBuilder};

fn main() {
    let house = WoodenHouseBuilder::new()
        .build_foundation()
        .build_structure()
        .build_roof()
        .build();

    house.show();
}
