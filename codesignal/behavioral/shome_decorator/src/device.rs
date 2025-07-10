// TODO: Define the trait Device
// - Define turn_on and turn_off methods
pub trait Device {
    fn turn_on(&self);
    fn turn_off(&self);
}

// TODO: Create the Light struct that implements Device
// - Implement turn_on and turn_off methods
pub struct Light;
impl Device for Light {
    fn turn_on(&self) {
        println!("Light is on");
    }
    fn turn_off(&self) {
        println!("Light is off");
    }
}

// TODO: Create the Fan struct that implements Device
// - Implement turn_on and turn_off methods
pub struct Fan {
    speed: u8,
}
impl Device for Fan {
    fn turn_on(&self) {
        println!("Fan is on");
    }
    fn turn_off(&self) {
        println!("Fan is off");
    }
}

// TODO: Implement speed as a private field with getter and setter in the Fan struct
impl Fan {
    pub fn new() -> Fan {
        Fan { speed: 0 }
    }
    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed;
    }
    pub fn get_speed(&self) -> u8 {
        self.speed
    }
}
