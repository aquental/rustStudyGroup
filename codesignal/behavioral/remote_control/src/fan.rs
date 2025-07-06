pub struct Fan {
    is_on: bool,
}

impl Fan {
    pub fn new() -> Self {
        Fan { is_on: false }
    }

    pub fn on(&mut self) {
        if !self.is_on {
            self.is_on = true;
            println!("Fan is on.");
        }
    }

    pub fn off(&mut self) {
        if self.is_on {
            self.is_on = false;
            println!("Fan is off.");
        }
    }
}
