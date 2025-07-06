// TODO: Implement a Fan struct that has on and off methods.
// - Create a struct named Fan with a boolean field `is_on`.
// - Implement an `on` method that sets `is_on` to true and prints "Fan is on.".
// - Implement an `off` method that sets `is_on` to false and prints "Fan is off.".

#[derive(Clone)]
pub struct Fan {
    is_on: bool,
}
impl Fan {
    pub fn new() -> Self {
        Fan { is_on: false }
    }

    pub fn on(&mut self) {
        self.is_on = true;
        println!("Fan is on.");
    }
    pub fn off(&mut self) {
        self.is_on = false;
        println!("Fan is off.");
    }
}
