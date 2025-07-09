use crate::security_device::SecurityDevice;

pub struct ThirdPartySensor;

impl ThirdPartySensor {
    pub fn detect_intrusion(&self) -> String {
        "Intrusion detected by third-party sensor!".to_string()
    }
}

// TODO: Define struct SensorAdapter with a field sensor: ThirdPartySensor
pub struct SensorAdapter {
    pub(crate) sensor: ThirdPartySensor,
}
impl SensorAdapter {
    pub fn detect_intrusion(&self) -> String {
        self.sensor.detect_intrusion()
    }
}

// TODO: Implement SecurityDevice for SensorAdapter
// - get_status should call sensor.detect_intrusion()
impl SecurityDevice for SensorAdapter {
    fn get_status(&self) -> String {
        self.detect_intrusion()
    }
}
