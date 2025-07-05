mod image;
mod image_adapter;
mod image_composite;

use image::{MacImage, WinImage};
use image_adapter::WinToMacAdapter;
use image_composite::ImageGallery;

fn main() {
    // TODO: Create instances of WinImage, MacImage, and WinToMacAdapter
    // - Instantiate a WinImage object
    // - Instantiate a MacImage object
    // - Instantiate a WinToMacAdapter object using the WinImage instance
    let win_image = WinImage::new();
    let mac_image = MacImage::new();
    let win_to_mac_adapter = WinToMacAdapter::new(win_image);

    // TODO: Create an instance of ImageGallery using instances of MacImage and WinToMacAdapter
    // - Instantiate an ImageGallery object
    // - Add the MacImage and WinToMacAdapter instances to the ImageGallery
    let mut image_gallery = ImageGallery::new();
    image_gallery.add(Box::new(mac_image));
    image_gallery.add(Box::new(win_to_mac_adapter));

    // TODO: Display the ImageGallery to show all the images
    // - Call the display() method of the ImageGallery object
    image_gallery.display();

    // TODO: Remove one image and display the ImageGallery again
    // - Remove an image from the ImageGallery
    // - Call the display() method of the ImageGallery object again
    image_gallery.remove(0);
    image_gallery.display();
}
