pub trait Exhibit {
    // TODO: Declare method to show exhibit details
    fn show_details(&self);
    // TODO: Declare method to get exhibit title
    fn get_title(&self) -> &str;
}

pub struct Painting {
    // TODO: Define fields for title and artist
    title: String,
    artist: String,
}

impl Painting {
    // TODO: Implement constructor to initialize title and artist
    pub fn new(title: String, artist: String) -> Self {
        // Implementation goes here
        Self { title, artist }
    }
}

impl Exhibit for Painting {
    // TODO: Override show_details to display painting's information in the format "<title> by <artist>"
    fn show_details(&self) {
        // Implementation goes here
        println!("{} by {}", self.title, self.artist);
    }

    // TODO: Override get_title to return the painting's title
    fn get_title(&self) -> &str {
        // Implementation goes here
        &self.title
    }
}

pub struct Gallery {
    // TODO: Declare a field to store the exhibits in the gallery
    store: Vec<Box<dyn Exhibit>>,
}

impl Gallery {
    // TODO: Implement a constructor
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }

    // TODO: Implement a method add(exhibit: Box<dyn Exhibit>) to add an exhibit to the gallery
    pub fn add(&mut self, exhibit: Box<dyn Exhibit>) {
        self.store.push(exhibit);
    }

    // TODO: Implement a method remove(exhibit_title: &str) to remove an exhibit from the gallery by title
    pub fn remove(&mut self, exhibit_title: &str) {
        self.store
            .retain(|exhibit| exhibit.get_title() != exhibit_title);
    }
}

impl Exhibit for Gallery {
    // TODO: Override show_details to display details of all exhibits in the gallery
    fn show_details(&self) {
        // Implementation goes here
        for exhibit in &self.store {
            exhibit.show_details();
        }
    }

    // TODO: Override get_title to return "Gallery"
    fn get_title(&self) -> &str {
        // Implementation goes here
        "Gallery"
    }
}
