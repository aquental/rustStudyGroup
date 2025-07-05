use crate::checkbox::{Checkbox, MacCheckbox};

pub struct MacToWinCheckboxAdapter {
    mac_checkbox: MacCheckbox,
}

impl MacToWinCheckboxAdapter {
    // TODO: Initialize the MacToWinCheckboxAdapter with a MacCheckbox
    // - Implement a constructor to initialize the mac_checkbox field.
    pub fn new(mac_checkbox: MacCheckbox) -> Self {
        Self { mac_checkbox }
    }
}

impl Checkbox for MacToWinCheckboxAdapter {
    // TODO: Implement the render method to call the MacCheckbox's render method
    // - Override the render method so that it calls mac_checkbox.render().
    fn render(&self) {
        self.mac_checkbox.render();
    }
}
