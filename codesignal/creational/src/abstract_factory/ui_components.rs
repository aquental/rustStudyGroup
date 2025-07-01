pub trait Slider {
    fn render(&self);
}

pub struct WinSlider;

impl Slider for WinSlider {
    fn render(&self) {
        println!("Rendering a slider in a Windows style.");
    }
}

pub struct MacSlider;

impl Slider for MacSlider {
    fn render(&self) {
        println!("Rendering a slider in a Mac style.");
    }
}

pub trait Toggle {
    fn render(&self);
}

pub struct WinToggle;

impl Toggle for WinToggle {
    fn render(&self) {
        println!("Rendering a toggle in a Windows style.");
    }
}

pub struct MacToggle;

impl Toggle for MacToggle {
    fn render(&self) {
        println!("Rendering a toggle in a Mac style.");
    }
}

pub trait UIComponentFactory {
    fn create_slider(&self) -> Box<dyn Slider>;
    fn create_toggle(&self) -> Box<dyn Toggle>;
}

pub struct WinUIFactory;

impl UIComponentFactory for WinUIFactory {
    fn create_slider(&self) -> Box<dyn Slider> {
        Box::new(WinSlider)
    }

    fn create_toggle(&self) -> Box<dyn Toggle> {
        Box::new(WinToggle)
    }
}

pub struct MacUIFactory;

impl UIComponentFactory for MacUIFactory {
    fn create_slider(&self) -> Box<dyn Slider> {
        Box::new(MacSlider)
    }

    fn create_toggle(&self) -> Box<dyn Toggle> {
        Box::new(MacToggle)
    }
}

// TODO: Declare the MobileApplication struct with a factory parameter
pub struct MobileApplication {
    // TODO: Declare a private Slider field and initialize it using 'factory.create_slider()'
    slider: Box<dyn Slider>,
    // TODO: Declare a private Toggle field and initialize it using 'factory.create_toggle()'
    toggle: Box<dyn Toggle>,
}

// TODO: Implement the render method
// - Call the render method from 'slider'
// - Call the render method from 'toggle'
impl MobileApplication {
    pub fn new(factory: &dyn UIComponentFactory) -> Self {
        MobileApplication {
            slider: factory.create_slider(),
            toggle: factory.create_toggle(),
        }
    }

    pub fn render(&self) {
        self.slider.render();
        self.toggle.render();
    }
}
