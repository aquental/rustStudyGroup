mod exhibit;

use exhibit::{Exhibit, Gallery, Painting};

fn main() {
    // TODO: Create Painting instances with given titles and artists
    // - "Starry Night" by Vincent van Gogh
    // - "Mona Lisa" by Leonardo da Vinci
    let painting1 = Painting::new("Starry Night".to_string(), "Vincent van Gogh".to_string());
    let painting2 = Painting::new("Mona Lisa".to_string(), "Leonardo da Vinci".to_string());

    // TODO: Create a Gallery instance
    let mut gallery = Gallery::new();

    // TODO: Add the Painting instances to the Gallery
    gallery.add(Box::new(painting1));
    gallery.add(Box::new(painting2));

    // TODO: Call the show_details method for the Gallery to display details of all exhibits
    gallery.show_details();

    // TODO: Remove one of the paintings from the Gallery by title
    gallery.remove("Mona Lisa");

    // TODO: Call the show_details method again for the Gallery to display details of the remaining exhibits
    gallery.show_details();
}
