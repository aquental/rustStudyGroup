use crate::device::Device;
// TODO: Create the ColoredLight struct to add color functionality to a Light
// - Implement Device trait
// - Add a constructor that takes a Device and a String color
// - Implement the turn_on method to turn on the light and change its color
// - Implement the turn_off method
pub struct ColoredLight {
    device: Box<dyn Device>,
    color: String,
}
impl ColoredLight {
    pub fn new(device: Box<dyn Device>, color: String) -> ColoredLight {
        ColoredLight { device, color }
    }
}
impl Device for ColoredLight {
    fn turn_on(&self) {
        println!("{} is on", self.color);
        self.device.turn_on();
    }
    fn turn_off(&self) {
        println!("{} is off", self.color);
        self.device.turn_off();
    }
}
