// TODO: Remember to add necessary use (import) statements for the food delivery types
use crate::food_delivery::{DoorDash, FoodDeliveryPlatform, GrubHub, UberEats};

// TODO: Create UberEatsAdapter struct implementing FoodDeliveryPlatform
// - Adapt the UberEats struct to the FoodDeliveryPlatform trait.
// - Ensure that the UberEats object is initialized properly within the constructor.
pub struct UberEatsAdapter {
    pub ueats: UberEats,
}
impl UberEatsAdapter {
    pub fn new(eats: UberEats) -> Self {
        Self { ueats: eats }
    }
}
impl FoodDeliveryPlatform for UberEatsAdapter {
    fn process_order(&self, order: &str) {
        self.ueats.process_order(order)
    }
}

// TODO: Create DoorDashAdapter struct implementing FoodDeliveryPlatform
// - Adapt the DoorDash struct to the FoodDeliveryPlatform trait.
// - Ensure that the DoorDash object is initialized properly within the constructor.
pub struct DoorDashAdapter {
    pub ddash: DoorDash,
}
impl DoorDashAdapter {
    pub fn new(dash: DoorDash) -> Self {
        Self { ddash: dash }
    }
}
impl FoodDeliveryPlatform for DoorDashAdapter {
    fn process_order(&self, order: &str) {
        self.ddash.process_order(order)
    }
}

// TODO: Create GrubHubAdapter struct implementing FoodDeliveryPlatform
// - Adapt the GrubHub struct to the FoodDeliveryPlatform trait.
// - Ensure that the GrubHub object is initialized properly within the constructor.
pub struct GrubHubAdapter {
    pub ghub: GrubHub,
}
impl GrubHubAdapter {
    pub fn new(hub: GrubHub) -> Self {
        Self { ghub: hub }
    }
}
impl FoodDeliveryPlatform for GrubHubAdapter {
    fn process_order(&self, order: &str) {
        self.ghub.process_order(order)
    }
}
