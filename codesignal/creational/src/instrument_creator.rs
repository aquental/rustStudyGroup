// TODO: Import the necessary structs/traits from the instrument module
use crate::instrument::{Drum, Guitar, Instrument, Piano};
// TODO: Create a trait named InstrumentCreator with a method 'create_instrument'.
pub(crate) trait InstrumentCreator {
    fn create_instrument(&self) -> Box<dyn Instrument>;
}
// TODO: Create a struct named GuitarCreator that implements InstrumentCreator and implements the 'create_instrument' method.
pub(crate) struct GuitarCreator;
impl InstrumentCreator for GuitarCreator {
    fn create_instrument(&self) -> Box<dyn Instrument> {
        println!("Creating the guitar!");
        Box::new(Guitar {})
    }
}
// TODO: Create a struct named PianoCreator that implements InstrumentCreator and implements the 'create_instrument' method.
pub(crate) struct PianoCreator;
impl InstrumentCreator for PianoCreator {
    fn create_instrument(&self) -> Box<dyn Instrument> {
        println!("Creating the piano!");
        Box::new(Piano {})
    }
}
// TODO: Create a struct named DrumCreator that implements InstrumentCreator and implements the 'create_instrument' method.
pub(crate) struct DrumCreator;
impl InstrumentCreator for DrumCreator {
    fn create_instrument(&self) -> Box<dyn Instrument> {
        println!("Creating the drum!");
        Box::new(Drum {})
    }
}
