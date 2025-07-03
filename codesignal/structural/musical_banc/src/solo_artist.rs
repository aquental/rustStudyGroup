use crate::musician::Musician;

// TODO: Define the SoloArtist struct with fields name and instrument (strings)
pub struct SoloArtist {
    name: String,
    instrument: String,
}

// TODO: Implement the new() method for SoloArtist
impl SoloArtist {
    pub fn new(name: String, instrument: String) -> SoloArtist {
        SoloArtist { name, instrument }
    }
}

// TODO: Implement the Musician trait for SoloArtist
impl Musician for SoloArtist {
    fn show_details(&self) {
        println!("Name: {}, Instrument: {}", self.name, self.instrument);
    }
}
