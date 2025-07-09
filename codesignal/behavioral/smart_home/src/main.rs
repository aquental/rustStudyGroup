mod device_factory;
mod security_device;
mod third_party_adapter;

use device_factory::{AlarmFactory, CameraFactory, DeviceFactory};
use security_device::SecurityDevice;
use third_party_adapter::{SensorAdapter, ThirdPartySensor};

fn main() {
    // TODO: Create an instance of AlarmFactory
    let factory = AlarmFactory;
    // TODO: Use the factory to create an Alarm device
    let alarm = factory.create_device();
    // TODO: Print the status of the Alarm device
    println!("Alarm status: {}", alarm.get_status());

    // TODO: Create an instance of CameraFactory
    let factory = CameraFactory;
    // TODO: Use the factory to create a Camera device
    let camera = factory.create_device();
    // TODO: Print the status of the Camera device
    println!("Camera status: {}", camera.get_status());

    // TODO: Create an instance of ThirdPartySensor
    let sensor = ThirdPartySensor;
    // TODO: Create an instance of SensorAdapter
    let adapter = SensorAdapter { sensor };
    // TODO: Print the status using the adapter
    println!("Sensor status: {}", adapter.get_status());
}
