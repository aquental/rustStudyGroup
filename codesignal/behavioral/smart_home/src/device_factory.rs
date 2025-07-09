use crate::security_device::{Alarm, Camera, SecurityDevice};

// TODO: Define a trait DeviceFactory with a method create_device(&self) -> Box<dyn SecurityDevice>
pub trait DeviceFactory {
    fn create_device(&self) -> Box<dyn SecurityDevice>;
}
// TODO: Implement struct AlarmFactory that implements DeviceFactory
// - create_device should return an instance of Alarm
pub struct AlarmFactory;

impl DeviceFactory for AlarmFactory {
    fn create_device(&self) -> Box<dyn SecurityDevice> {
        Box::new(Alarm {})
    }
}

// TODO: Implement struct CameraFactory that implements DeviceFactory
// - create_device should return an instance of Camera
pub struct CameraFactory;

impl DeviceFactory for CameraFactory {
    fn create_device(&self) -> Box<dyn SecurityDevice> {
        Box::new(Camera {})
    }
}
