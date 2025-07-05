pub trait Checkbox {
    // TODO: Define the Checkbox trait with an abstract render method
    fn render(&self);
}

pub struct WinCheckbox;

impl Checkbox for WinCheckbox {
    // TODO: Override the render method to display "Rendering a checkbox in a Windows style."
    fn render(&self) {
        println!("Rendering a checkbox in a Windows style.");
    }
}

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    // TODO: Override the render method to display "Rendering a checkbox in a Mac style."
    fn render(&self) {
        println!("Rendering a checkbox in a Mac style.");
    }
}
