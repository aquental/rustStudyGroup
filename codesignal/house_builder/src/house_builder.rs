use crate::house::House;

// TODO: Define the trait HouseBuilder with methods:
pub trait HouseBuilder {
    fn build_foundation(self) -> Self;
    fn build_structure(self) -> Self;
    fn build_roof(self) -> Self;
    fn build(self) -> House;
}

pub struct ConcreteHouseBuilder {
    // TODO: Initialize a private House object.
    foundation: String,
    structure: String,
    roof: String,
}

impl ConcreteHouseBuilder {
    pub fn new() -> Self {
        Self {
            // TODO: Initialize the House object.
            foundation: String::new(),
            structure: String::new(),
            roof: String::new(),
        }
    }
}

// TODO: Implement the HouseBuilder trait for ConcreteHouseBuilder.
// - Each method should modify the house and return self to enable method chaining.
impl HouseBuilder for ConcreteHouseBuilder {
    fn build_foundation(self) -> Self {
        ConcreteHouseBuilder {
            foundation: "Concrete".to_string(),
            structure: self.structure,
            roof: self.roof,
        }
    }
    fn build_structure(self) -> Self {
        ConcreteHouseBuilder {
            foundation: self.foundation,
            structure: "Concrete".to_string(),
            roof: self.roof,
        }
    }
    fn build_roof(self) -> Self {
        ConcreteHouseBuilder {
            foundation: self.foundation,
            structure: self.structure,
            roof: "Concrete".to_string(),
        }
    }
    fn build(self) -> House {
        House {
            foundation: self.foundation,
            structure: self.structure,
            roof: self.roof,
        }
    }
}
