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
