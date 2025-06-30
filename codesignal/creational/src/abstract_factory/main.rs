mod application;
mod button;
mod checkbox;
mod factory;

use factory::{GUIFactory, LinuxFactory, MacFactory, WinFactory};
use application::Application;

fn main() {
    let os : &str = "Linux"; // Can be "Windows", "Mac", or "Linux"
    
    let factory: Box<dyn GUIFactory> = match os {
        "Windows" => Box::new(WinFactory),
        "Mac" => Box::new(MacFactory),
        // TODO: Handle the case for "Linux" by returning LinuxFactory
        "Linux" => Box::new(LinuxFactory),
        _ => panic!("Unknown operating system"),
    };

    let app = Application::new(factory);
    app.paint();
}
