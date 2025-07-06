// TODO: Define the TV struct with the following:
// - Fields to track if the TV is on or off and the current channel
// Implement the following methods:
// - on(): turns the TV on and prints "TV is on."
// - off(): turns the TV off and prints "TV is off."
// - get_current_channel(&self) -> u32: returns the current channel
// - change_channel(channel: u32): changes the channel if the TV is on and prints "TV channel changed to {channel}."
#[derive(Clone)]
pub struct TV {
    is_on: bool,
    current_channel: u32,
}
impl TV {
    pub fn new() -> Self {
        TV {
            is_on: false,
            current_channel: 1,
        }
    }
    pub fn on(&mut self) {
        self.is_on = true;
        println!("TV is on.");
    }
    pub fn off(&mut self) {
        self.is_on = false;
        println!("TV is off.");
    }
    pub fn get_current_channel(&self) -> u32 {
        self.current_channel
    }
    pub fn change_channel(&mut self, channel: u32) {
        if self.is_on {
            self.current_channel = channel;
            println!("TV channel changed to {channel}.")
        }
    }
}
