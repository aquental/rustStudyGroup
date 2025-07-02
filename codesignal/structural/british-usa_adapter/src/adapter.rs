// British plug struct
pub struct BritishPlug;

impl BritishPlug {
    pub fn engage(&self) {
        // Printing British plug engaged message
        println!("British plug engaged.");
    }
}

// US plug trait
pub trait USPlug {
    fn plug_in(&self);
}

// Adapter struct implementing USPlug trait
pub struct BritishToUSAdapter {
    // TODO: Define field of type BritishPlug
    british_plug: BritishPlug,
}

impl BritishToUSAdapter {
    // TODO: Implement a method to initialize the BritishPlug instance
    pub fn new(british_plug: BritishPlug) -> Self {
        BritishToUSAdapter { british_plug }
    }
}

impl USPlug for BritishToUSAdapter {
    fn plug_in(&self) {
        // TODO: Implement the plug_in method to call engage method of BritishPlug
        self.british_plug.engage();
    }
}
