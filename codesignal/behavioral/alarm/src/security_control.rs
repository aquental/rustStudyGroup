// TODO: Define the AlarmListener trait
// - Define the on_alarm method
pub trait AlarmListener {
    fn on_alarm(&self);
}

// TODO: Define the SecurityControl struct
// - listeners: Vec<Box<dyn AlarmListener>>
pub struct SecurityControl {
    listeners: Vec<Box<dyn AlarmListener>>,
}

// TODO: Implement methods for SecurityControl
// - new() -> Self
// - add_listener(&mut self, listener: Box<dyn AlarmListener>)
// - trigger_alarm(&self)
//   - Remember to call listener.on_alarm() for each listener in the listeners vector
impl SecurityControl {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn AlarmListener>) {
        self.listeners.push(listener);
    }

    pub fn trigger_alarm(&self) {
        for listener in &self.listeners {
            listener.on_alarm();
        }
    }
}

// TODO: Define the FireSensor struct
// - Implement AlarmListener for FireSensor
//   - on_alarm method prints "Fire sensor activated! Evacuate immediately!"
pub struct FireSensor;
impl AlarmListener for FireSensor {
    fn on_alarm(&self) {
        println!("Fire sensor activated! Evacuate immediately!");
    }
}

// TODO: Define the GasSensor struct
// - Implement AlarmListener for GasSensor
//   - on_alarm method prints "Gas sensor detected a leak! Shut off the gas supply!"
pub struct GasSensor;
impl AlarmListener for GasSensor {
    fn on_alarm(&self) {
        println!("Gas sensor detected a leak! Shut off the gas supply!");
    }
}
