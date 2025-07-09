use crate::brightness_strategy::BrightnessStrategy;

pub trait Device {
    fn turn_on(&self);
    fn turn_off(&self);
}

// TODO: Implement the Light struct that implements Device
// - Light should have a brightness strategy
// - Implement turn_on and turn_off methods
pub struct Light {
    brightness_strategy: Box<dyn BrightnessStrategy>,
}

impl Light {
    pub fn new(brightness_strategy: Box<dyn BrightnessStrategy>) -> Light {
        Light { brightness_strategy }
    }
}
impl Device for Light {
    fn turn_on(&self) {
        self.brightness_strategy.adjust_brightness();
        println!("Light is on.");
    }
    fn turn_off(&self) {
        println!("Light is off.");
    }
}

// TODO: Implement the Fan struct that implements Device
// - Implement turn_on method to print "Fan is on."
// - Implement turn_off method to print "Fan is off."
pub struct Fan;

impl Fan {
    pub fn new() -> Fan {
        Fan
    }
}
impl Device for Fan {
    fn turn_on(&self) {
        println!("Fan is on.");
    }
    fn turn_off(&self) {
        println!("Fan is off.");
    }
}
