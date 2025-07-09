mod device;
mod device_factory;
mod brightness_strategy;

use device::Device;
use device_factory::{create_fan, create_light};
use brightness_strategy::{BrightStrategy, DimStrategy};

fn main() {
    // TODO: Create a light with BrightStrategy using the factory
    let bright_light = create_light(Box::new(BrightStrategy));
    // TODO: Turn on the light
    bright_light.turn_on();
    // TODO: Turn off the light
    bright_light.turn_off();

    println!();

    // TODO: Create a light with DimStrategy using the factory
    let dim_light = create_light(Box::new(DimStrategy));
    // TODO: Turn on the light
    dim_light.turn_on();
    // TODO: Turn off the light
    dim_light.turn_off();

    println!();

    // TODO: Create a fan using the factory
    let fan = create_fan();
    // TODO: Turn on the fan
    fan.turn_on();
    // TODO: Turn off the fan
    fan.turn_off();
}
