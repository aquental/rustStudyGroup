mod ui_components;
use ui_components::{MacUIFactory, MobileApplication, UIComponentFactory, WinUIFactory};

fn main() {
    let os_type = "Windows"; // Change to "Mac" for MacUIFactory

    let factory: Option<Box<dyn UIComponentFactory>> = match os_type {
        "Windows" => Some(Box::new(WinUIFactory)),
        "Mac" => Some(Box::new(MacUIFactory)),
        _ => None,
    };

    match factory {
        Some(f) => {
            // TODO: Create a MobileApplication object 'app' using the factory
            let app = MobileApplication::new(&*f);
            // TODO: Render the UI components by calling the render method
            app.render();
        }
        None => eprintln!("Unknown OS type."),
    }
}
