mod instrument;
mod instrument_creator;

// TODO: Import the necessary structs/traits from instrument and instrument_creator modules
use instrument_creator::{DrumCreator, GuitarCreator, InstrumentCreator, PianoCreator};

fn main() {
    // TODO: Instantiate GuitarCreator, use it to create a Guitar object, and call the 'play' method.
    let guitar_creator = GuitarCreator;
    let g = guitar_creator.create_instrument();
    g.play();

    // TODO: Instantiate PianoCreator, use it to create a Piano object, and call the 'play' method.
    let piano_creator = PianoCreator;
    let p = piano_creator.create_instrument();
    p.play();

    // TODO: Instantiate DrumCreator, use it to create a Drum object, and call the 'play' method.
    let drum_creator = DrumCreator;
    let d = drum_creator.create_instrument();
    d.play();
}
