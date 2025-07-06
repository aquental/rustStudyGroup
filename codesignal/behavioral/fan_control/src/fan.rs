// TODO: Implement the Fan struct with low, medium, high, and off methods
// - Define a struct named `Fan`.
// - Inside `Fan`, implement methods: `low()`, `medium()`, `high()`, and `off()` that change the fan's speed and print a message indicating the current speed.

pub struct Fan {
    speed: u32,
}
impl Fan {
    pub fn new() -> Self {
        Fan { speed: 0 }
    }
    pub fn low(&mut self) {
        self.speed = 1;
        println!("Fan speed: low ({})", self.speed);
    }
    pub fn medium(&mut self) {
        self.speed = 2;
        println!("Fan speed: medium ({})", self.speed);
    }
    pub fn high(&mut self) {
        self.speed = 3;
        println!("Fan speed: high ({})", self.speed);
    }
    pub fn off(&mut self) {
        self.speed = 0;
        println!("Fan speed: off ({})", self.speed);
    }
}
