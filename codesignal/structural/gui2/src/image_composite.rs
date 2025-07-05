use crate::image::BaseImage;
use std::vec::Vec;

pub struct ImageGallery {
    images: Vec<Box<dyn BaseImage>>,
}

// TODO: Implement methods for ImageGallery
// - Implement an add(image: Box<dyn BaseImage>) method
// - The method should add the provided image to the images vector
// - Implement a remove(index: usize) method
// - The method should remove the image at the specified index from the images vector
// - Implement a display() method
// - The method should iterate over each image in the images vector
// - It should call the display method on each image
impl ImageGallery {
    pub fn new() -> ImageGallery {
        ImageGallery {
            images: Vec::new(),
        }
    }
    pub fn add(&mut self, image: Box<dyn BaseImage>) {
        self.images.push(image);
    }
    pub fn remove(&mut self, index: usize) {
        if index < self.images.len() {
            self.images.remove(index);
        }
    }
    pub fn display(&self) {
        for image in &self.images {
            image.display();
        }
    }
}
