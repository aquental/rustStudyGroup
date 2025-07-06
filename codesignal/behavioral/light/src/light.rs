pub struct Light {
    is_on: bool,
}

impl Light {
    pub fn new() -> Self {
        Light { is_on: false }
    }

    pub fn on(&mut self) {
        println!("Light is on.");
        self.is_on = true;
    }

    pub fn off(&mut self) {
        println!("Light is off.");
        self.is_on = false;
    }
}
