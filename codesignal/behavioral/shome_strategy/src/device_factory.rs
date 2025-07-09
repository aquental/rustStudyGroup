use crate::brightness_strategy::BrightnessStrategy;
use crate::device::{Fan, Light};

// TODO: Implement functions create_light and create_fan
// - create_light should return a new Light with the given brightness strategy
// - create_fan should return a new Fan

pub fn create_light(brightness_strategy: Box<dyn BrightnessStrategy>) -> Light {
    Light::new(brightness_strategy)
}

pub fn create_fan() -> Fan {
    Fan::new()
}
