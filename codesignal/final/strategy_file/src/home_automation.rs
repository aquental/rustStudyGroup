// Existing lighting system with switch_on method
struct OldLightSystem {}

impl OldLightSystem {
    fn new() -> Self {
        OldLightSystem {}
    }

    fn switch_on(&self) {
        println!("Old Light System: Light switched ON");
    }
}

// New trait expected by the home automation system
trait LightController {
    fn turn_on(&self);
}

// Adapter to make OldLightSystem compatible with LightController trait
struct LightSystemAdapter {
    old_system: OldLightSystem,
}

impl LightSystemAdapter {
    fn new(old_system: OldLightSystem) -> Self {
        LightSystemAdapter { old_system }
    }
}

impl LightController for LightSystemAdapter {
    fn turn_on(&self) {
        // Delegate to the old system's switch_on method
        self.old_system.switch_on();
    }
}

fn main() {
    // Create an instance of the old lighting system
    let old_light = OldLightSystem::new();

    // Wrap it with the adapter to make it compatible with LightController
    let adapted_light = LightSystemAdapter::new(old_light);

    // Test the new interface
    adapted_light.turn_on();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_old_light_system_switch_on() {
        let old_light = OldLightSystem::new();
        old_light.switch_on(); // Should print to stdout
    }

    #[test]
    fn test_light_system_adapter_turn_on() {
        let old_light = OldLightSystem::new();
        let adapter = LightSystemAdapter::new(old_light);
        adapter.turn_on(); // Should print to stdout
    }
}
