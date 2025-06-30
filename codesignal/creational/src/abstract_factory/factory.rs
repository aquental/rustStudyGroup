use super::button::{Button, WinButton, MacButton, LinuxButton};
use super::checkbox::{Checkbox, WinCheckbox, MacCheckbox, LinuxCheckbox};

pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

pub struct WinFactory;

impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox)
    }
}

pub struct MacFactory;

impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}

pub struct LinuxFactory;

// TODO: Implement LinuxFactory struct implementing GUIFactory
// - Implement create_button method to return LinuxButton()
// - Implement create_checkbox method to return LinuxCheckbox()
impl GUIFactory for LinuxFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(LinuxButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(LinuxCheckbox)
    }
}
