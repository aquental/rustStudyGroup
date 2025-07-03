use crate::musician::Musician;

// TODO: Define the Band struct implementing the Musician trait.
// - Implement methods to:
//   - Add musicians
//   - Show their details using Vec<Box<dyn Musician>>.
// Define the Band struct
pub struct Band {
    name: String,
    details: Vec<Box<dyn Musician>>,
    genre: String,
}

impl Band {
    pub fn new(name: String, genre: String) -> Band {
        Band {
            name,
            details: Vec::new(),
            genre,
        }
    }
    pub fn add_musicians(&mut self, musician: Box<dyn Musician>) {
        self.details.push(musician);
    }
    pub fn show_details(&self) {
        for musician in &self.details {
            musician.show_details();
        }
    }
}

// Implement the Musician trait for Band
impl Musician for Band {
    fn show_details(&self) {
        println!("Band: {}, Genre: {}", self.name, self.genre);
    }
}
