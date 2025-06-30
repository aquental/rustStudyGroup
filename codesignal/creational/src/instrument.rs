// TODO: Create a trait named Instrument with a method 'play'.
pub trait Instrument {
    fn play(&self);
}

// TODO: Create a struct named Guitar that implements Instrument and implements the 'play' method.
pub struct Guitar;

impl Instrument for Guitar {
    fn play(&self) {
        println!("Strumming the guitar!");
    }
}
// TODO: Create a struct named Piano that implements Instrument and implements the 'play' method.
pub struct Piano;

impl Instrument for Piano {
    fn play(&self) {
        println!("Playing the piano keys!");
    }
}
// TODO: Create a struct named Drum that implements Instrument and implements the 'play' method.
pub struct Drum;

impl Instrument for Drum {
    fn play(&self) {
        println!("Beating the drums!");
    }
}
