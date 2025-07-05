// TODO: Define the trait FoodDeliveryPlatform with a method process_order
// - Define a trait with a single method signature: process_order(order: &str).
pub trait FoodDeliveryPlatform {
    fn process_order(&self, order: &str);
}

// TODO: Implement UberEats struct with appropriate order processing method
// - Implement the UberEats struct with a method place_order(order: &str).
pub struct UberEats;
impl UberEats {
    fn place_order(&self, order: &str) {
        println!("Placing order: {}", order);
    }
    pub fn new() -> Self {
        Self
    }
}
impl FoodDeliveryPlatform for UberEats {
    fn process_order(&self, order: &str) {
        self.place_order(order);
    }
}

// TODO: Implement DoorDash struct with appropriate order processing method
// - Implement the DoorDash struct with a method submit_order(order: &str).
pub struct DoorDash;
impl DoorDash {
    fn submit_order(&self, order: &str) {
        println!("Submitting order: {}", order);
    }
    pub fn new() -> Self {
        Self
    }
}
impl FoodDeliveryPlatform for DoorDash {
    fn process_order(&self, order: &str) {
        self.submit_order(order);
    }
}
// TODO: Implement GrubHub struct with appropriate order processing method
// - Implement the GrubHub struct with a method execute_order(order: &str).
pub struct GrubHub;
impl GrubHub {
    fn execute_order(&self, order: &str) {
        println!("Executing order: {}", order);
    }
    pub fn new() -> Self {
        Self
    }
}
impl FoodDeliveryPlatform for GrubHub {
    fn process_order(&self, order: &str) {
        self.execute_order(order);
    }
}
