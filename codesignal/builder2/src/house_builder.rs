use crate::house::House;

pub trait HouseBuilder {
    fn build_foundation(self) -> Self;
    fn build_structure(self) -> Self;
    fn build_roof(self) -> Self;
    fn build(self) -> House;
}

pub struct WoodenHouseBuilder {
    // TODO: Implement the WoodenHouseBuilder
    // - The struct should have fields for foundation, structure, and roof
    pub foundation: String,
    pub structure: String,
    pub roof: String,
}

impl WoodenHouseBuilder {
    // TODO: Implement the constructor to initialize the builder
    // - Initialize the fields with empty strings
    pub fn new() -> Self {
        WoodenHouseBuilder {
            foundation: "".to_string(),
            structure: "".to_string(),
            roof: "".to_string(),
        }
    }
}

impl HouseBuilder for WoodenHouseBuilder {
    // TODO: Implement the build_foundation method
    // - Set the foundation field to "Wooden Foundation"
    // - Return the current instance (Self)
    fn build_foundation(self) -> Self {
        WoodenHouseBuilder {
            foundation: "Wooden Foundation".to_string(),
            structure: self.structure,
            roof: self.roof,
        }
    }

    // TODO: Implement the build_structure method
    // - Set the structure field to "Wooden Structure"
    // - Return the current instance (Self)
    fn build_structure(self) -> Self {
        WoodenHouseBuilder {
            foundation: self.foundation,
            structure: "Wooden Structure".to_string(),
            roof: self.roof,
        }
    }

    // TODO: Implement the build_roof method
    // - Set the roof field to "Wooden Roof"
    // - Return the current instance (Self)
    fn build_roof(self) -> Self {
        WoodenHouseBuilder {
            foundation: self.foundation,
            structure: self.structure,
            roof: "Wooden Roof".to_string(),
        }
    }
    // TODO: Implement the build method
    // - Return a new House using the builder's field values for foundation, structure, and roof
    fn build(self) -> House {
        House {
            foundation: self.foundation,
            structure: self.structure,
            roof: self.roof,
        }
    }
}
