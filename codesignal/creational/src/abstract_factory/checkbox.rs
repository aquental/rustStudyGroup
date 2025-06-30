pub trait Checkbox {
    fn paint(&self);
}

pub struct WinCheckbox;

impl Checkbox for WinCheckbox {
    fn paint(&self) {
        println!("Rendering a checkbox in a Windows style.");
    }
}

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn paint(&self) {
        println!("Rendering a checkbox in a Mac style.");
    }
}

// TODO: Implement LinuxCheckbox struct implementing Checkbox
// - Implement the paint method to print "Rendering a checkbox in a Linux style."
pub struct LinuxCheckbox;

impl Checkbox for LinuxCheckbox {
    fn paint(&self) {
        println!("Rendering a checkbox in a Linux style.");
    }
}
