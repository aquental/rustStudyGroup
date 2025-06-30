use crate::controller::{Controller, PlayStationController, XboxController};
use crate::headset::{Headset, PlayStationHeadset, XboxHeadset};

pub trait AccessoryFactory {
    fn create_controller(&self) -> Box<dyn Controller>;
    fn create_headset(&self) -> Box<dyn Headset>;
}

// TODO: Create a PlayStationFactory struct that implements AccessoryFactory
pub struct PlayStationFactory;
impl AccessoryFactory for PlayStationFactory {
    // - Implement the create_controller method to return a new instance of PlayStationController.
    fn create_controller(&self) -> Box<dyn Controller> {
        Box::new(PlayStationController)
    }
    // - Implement the create_headset method to return a new instance of PlayStationHeadset.
    fn create_headset(&self) -> Box<dyn Headset> {
        Box::new(PlayStationHeadset)
    }
}
// TODO: Create an XboxFactory struct that implements AccessoryFactory
pub struct XboxFactory;
impl AccessoryFactory for XboxFactory {
    // - Implement the create_controller method to return a new instance of XboxController.
    fn create_controller(&self) -> Box<dyn Controller> {
        Box::new(XboxController)
    }
    // - Implement the create_headset method to return a new instance of XboxHeadset.
    fn create_headset(&self) -> Box<dyn Headset> {
        Box::new(XboxHeadset)
    }
}
