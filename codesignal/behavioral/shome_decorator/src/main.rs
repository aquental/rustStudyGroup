mod colored_light;
mod device;

use colored_light::ColoredLight;
use device::{Device, Fan, Light};

fn main() {
    // TODO: Create an instance of Light
    // TODO: Turn on the Light
    // TODO: Turn off the Light
    let light = Light {};
    light.turn_on();
    light.turn_off();

    // TODO: Instantiate ColoredLight with a Light and color "Red"
    // TODO: Turn on the ColoredLight
    // TODO: Turn off the ColoredLight
    let colored_light = ColoredLight::new(Box::new(Light {}), "Red".to_string());
    colored_light.turn_on();
    colored_light.turn_off();

    // TODO: Create an instance of Fan
    // TODO: Turn on the Fan
    // TODO: Optionally, set the Fan speed
    // TODO: Turn off the Fan
    let mut fan = Fan::new();
    fan.turn_on();
    fan.set_speed(5);
    fan.turn_off();
}
