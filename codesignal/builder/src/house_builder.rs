#[derive(Debug)]
pub struct House {
    foundation: String,
    structure: String,
    roof: String,
}

impl House {
    pub fn show(&self) {
        println!(
            "House with {}, {}, and {}.",
            self.foundation, self.structure, self.roof
        );
    }
}

pub trait HouseBuilder {
    fn build_foundation(&mut self) -> &mut Self;
    fn build_structure(&mut self) -> &mut Self;
    fn build_roof(&mut self) -> &mut Self;
    fn build(&self) -> House;
}

// TODO: Modify this struct to BrickHouseBuilder to build a Brick House
pub struct ConcreteHouseBuilder {
    house: House,
}

impl ConcreteHouseBuilder {
    pub fn new() -> Self {
        ConcreteHouseBuilder {
            house: House {
                foundation: String::new(),
                structure: String::new(),
                roof: String::new(),
            },
        }
    }
}

impl HouseBuilder for ConcreteHouseBuilder {
    fn build_foundation(&mut self) -> &mut Self {
        self.house.foundation = "Concrete Foundation".to_string();
        self
    }

    fn build_structure(&mut self) -> &mut Self {
        self.house.structure = "Concrete Structure".to_string();
        self
    }

    fn build_roof(&mut self) -> &mut Self {
        self.house.roof = "Concrete Roof".to_string();
        self
    }

    fn build(&self) -> House {
        House {
            foundation: self.house.foundation.clone(),
            structure: self.house.structure.clone(),
            roof: self.house.roof.clone(),
        }
    }
}


pub struct BrickHouseBuilder {
    house: House,
}

impl BrickHouseBuilder {
    pub fn new() -> Self {
        BrickHouseBuilder {
            house: House {
                foundation: String::new(),
                structure: String::new(),
                roof: String::new(),
            },
        }
    }
}

impl HouseBuilder for BrickHouseBuilder {
    fn build_foundation(&mut self) -> &mut Self {
        self.house.foundation = "Brick Foundation".to_string();
        self
    }

    fn build_structure(&mut self) -> &mut Self {
        self.house.structure = "Brick Structure".to_string();
        self
    }

    fn build_roof(&mut self) -> &mut Self {
        self.house.roof = "Brick Roof".to_string();
        self
    }

    fn build(&self) -> House {
        House {
            foundation: self.house.foundation.clone(),
            structure: self.house.structure.clone(),
            roof: self.house.roof.clone(),
        }
    }
}
