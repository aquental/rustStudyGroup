// TODO: Define the ClimateDevice trait
// - Include a get_status method that returns a String
pub trait ClimateDevice {
    fn get_status(&self) -> String;
}

// TODO: Implement the Heater struct that implements ClimateDevice
// - Implement get_status: return "Heater is warming the room to 70°F."
pub struct Heater;
impl ClimateDevice for Heater {
    fn get_status(&self) -> String {
        "Heater is warming the room to 70°F.".to_string()
    }
}

// TODO: Implement the AirConditioner struct that implements ClimateDevice
// - Implement get_status: return "Air Conditioner is cooling the room to 65°F."
pub struct AirConditioner;
impl ClimateDevice for AirConditioner {
    fn get_status(&self) -> String {
        "Air Conditioner is cooling the room to 65°F.".to_string()
    }
}

// TODO: Define the DeviceFactory trait
// - Include a create_device method that returns Box<dyn ClimateDevice>
pub trait DeviceFactory {
    fn create_device(&self) -> Box<dyn ClimateDevice>;
}

// TODO: Implement HeaterFactory that implements DeviceFactory
// - Implement create_device: return a new Heater instance
pub struct HeaterFactory;
impl DeviceFactory for HeaterFactory {
    fn create_device(&self) -> Box<dyn ClimateDevice> {
        Box::new(Heater)
    }
}

// TODO: Implement AirConditionerFactory that implements DeviceFactory
// - Implement create_device: return a new AirConditioner instance
pub struct AirConditionerFactory;
impl DeviceFactory for AirConditionerFactory {
    fn create_device(&self) -> Box<dyn ClimateDevice> {
        Box::new(AirConditioner)
    }
}
