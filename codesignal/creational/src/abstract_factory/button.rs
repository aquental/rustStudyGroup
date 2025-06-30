pub trait Button {
    fn paint(&self);
}

pub struct WinButton;

impl Button for WinButton {
    fn paint(&self) {
        println!("Rendering a button in a Windows style.");
    }
}

pub struct MacButton;

impl Button for MacButton {
    fn paint(&self) {
        println!("Rendering a button in a Mac style.");
    }
}

// TODO: Implement LinuxButton struct implementing Button
// - Implement the paint method to print "Rendering a button in a Linux style."
pub struct LinuxButton;

impl Button for LinuxButton {
    fn paint(&self) {
        println!("Rendering a button in a Linux style.");
    }
}
