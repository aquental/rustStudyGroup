// TODO: Define a trait SecurityDevice with a method get_status(&self) -> String
pub trait SecurityDevice {
    fn get_status(&self) -> String;
}
// TODO: Implement struct Alarm that implements SecurityDevice
// - The get_status method should return "Alarm is armed"
pub struct Alarm;
impl SecurityDevice for Alarm {
    fn get_status(&self) -> String {
        "Alarm is armed".to_string()
    }
}
// TODO: Implement struct Camera that implements SecurityDevice
// - The get_status method should return "Camera is recording"
pub struct Camera;
impl SecurityDevice for Camera {
    fn get_status(&self) -> String {
        "Camera is recording".to_string()
    }
}
