mod band;
mod musician;
mod solo_artist;

use band::Band;
use solo_artist::SoloArtist;

fn main() {
    // TODO: Create two SoloArtist instances:
    // - "John Doe" with "guitar"
    // - "Jane Smith" with "keyboard"
    let john_doe = SoloArtist::new("John Doe".to_string(), "guitar".to_string());
    let jane_smith = SoloArtist::new("Jane Smith".to_string(), "keyboard".to_string());

    // TODO: Create a Band instance and add the two SoloArtists to the band.
    let mut band = Band::new("The Band".to_string(), "Rock".to_string());
    band.add_musicians(Box::new(john_doe));
    band.add_musicians(Box::new(jane_smith));

    // TODO: Show the details of all musicians in the band.
    band.show_details();
}
