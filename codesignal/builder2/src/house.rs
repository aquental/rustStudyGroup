#[derive(Debug)]
pub struct House {
    pub foundation: String,
    pub structure: String,
    pub roof: String,
}

impl House {
    pub fn show(&self) {
        println!(
            "House with {}, {}, and {}.",
            self.foundation, self.structure, self.roof
        );
    }
}
