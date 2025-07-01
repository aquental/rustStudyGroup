mod house;
mod house_builder;

use house_builder::ConcreteHouseBuilder;
use house_builder::HouseBuilder;

fn main() {
    // TODO: Use the ConcreteHouseBuilder to construct a house using method chaining
    // and display the house details
    let house = ConcreteHouseBuilder::new()
        .build_foundation()
        .build_structure()
        .build_roof()
        .build();

    house.show();
}
