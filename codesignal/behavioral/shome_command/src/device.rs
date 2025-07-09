// TODO: Create a trait Device
// - Define on and off methods
pub trait Device {
    fn on(&mut self);
    fn off(&mut self);
}

// TODO: Create the Light struct that implements Device
// - Include a private bool variable 'is_on' to track the light's state
// - Implement the on and off methods
pub struct Light {
    is_on: bool,
}
impl Light {
    pub fn new() -> Light {
        Light { is_on: false }
    }
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}
impl Device for Light {
    fn on(&mut self) {
        self.is_on = true;
        println!("Light is now ON");
    }
    fn off(&mut self) {
        self.is_on = false;
        println!("Light is now OFF");
    }
}

// TODO: Create the Fan struct that implements Device
// - Include a private bool variable 'is_on' to track the fan's state
// - Include a private u32 variable 'speed' to control fan speed
// - Implement the on and off methods
// - Implement a speed property in the Fan struct
// - Provide a getter for the speed
// - Provide a setter for the speed
pub struct Fan {
    is_on: bool,
    speed: u32,
}
impl Fan {
    pub fn new() -> Fan {
        Fan {
            is_on: false,
            speed: 0,
        }
    }
}
impl Fan {
    pub fn get_speed(&self) -> u32 {
        self.speed
    }
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
}
impl Device for Fan {
    fn on(&mut self) {
        self.is_on = true;
        println!("Fan is now ON");
    }
    fn off(&mut self) {
        self.is_on = false;
        println!("Fan is now OFF");
    }
}
