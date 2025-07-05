pub trait BaseImage {
    fn display(&self);
}

pub struct WinImage;

impl WinImage {
    pub fn new() -> Self {
        Self
    }
}

// TODO: Implement BaseImage trait for WinImage
// - Override the display method
// - The method should print "Displaying a Windows-style image."
impl BaseImage for WinImage {
    fn display(&self) {
        println!("Displaying a Windows-style image.");
    }
}

pub struct MacImage;

impl MacImage {
    pub fn new() -> Self {
        Self
    }
}

// TODO: Implement BaseImage trait for MacImage
// - Override the display method
// - The method should print "Displaying a Mac-style image."
impl BaseImage for MacImage {
    fn display(&self) {
        println!("Displaying a Mac-style image.");
    }
}
