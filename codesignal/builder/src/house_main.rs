mod house_builder;

use house_builder::{BrickHouseBuilder, ConcreteHouseBuilder, HouseBuilder}; // TODO: Change this to BrickHouseBuilder

fn main() {
    let mut concrete_builder = ConcreteHouseBuilder::new();
    let chouse = concrete_builder
        .build_foundation()
        .build_structure()
        .build_roof()
        .build();
    chouse.show();

    let mut brick_builder = BrickHouseBuilder::new();
    let bhouse = brick_builder
        .build_foundation()
        .build_structure()
        .build_roof()
        .build();
    bhouse.show();
}
