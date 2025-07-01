#[derive(Default)]
pub struct House {
    // TODO: Define private properties for foundation, structure, and roof of type String.
    pub foundation: String,
    pub structure: String,
    pub roof: String,
}

impl House {
    // TODO: Implement setter methods: set_foundation, set_structure, set_roof.
    fn set_foundation(&mut self, foundation: String) {
        self.foundation = foundation;
    }
    fn set_structure(&mut self, structure: String) {
        self.structure = structure;
    }
    fn set_roof(&mut self, roof: String) {
        self.roof = roof;
    }

    // TODO: Implement a method `show` to display the house details in the format
    // "House with [foundation], [structure], and [roof]."
    pub fn show(&self) {
        println!(
            "House with {}, {}, and {}.",
            self.foundation, self.structure, self.roof
        );
    }
}
