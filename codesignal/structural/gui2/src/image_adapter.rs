use crate::image::{BaseImage, WinImage};

pub struct WinToMacAdapter {
    win_image: WinImage,
}

impl WinToMacAdapter {
    pub fn new(win_image: WinImage) -> Self {
        WinToMacAdapter { win_image }
    }
}

// TODO: Implement BaseImage trait for WinToMacAdapter
// - Override the display method
// - The method should call display method of the WinImage object
impl BaseImage for WinToMacAdapter {
    fn display(&self) {
        self.win_image.display();
    }
}
