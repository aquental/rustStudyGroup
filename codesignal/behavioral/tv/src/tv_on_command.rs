use crate::{command::Command, tv::TV};

// TODO: Define the TVOnCommand struct and implement Command for it
// - The execute method should turn the TV on
// - The undo method should turn the TV off and print "Undo: TV turned off."
pub struct TVOnCommand {
    tv: TV,
}
impl TVOnCommand {
    pub fn new(tv: TV) -> Self {
        TVOnCommand { tv }
    }
}
impl Command for TVOnCommand {
    fn execute(&mut self, tv: &mut TV) {
        tv.on();
    }
    fn undo(&mut self, tv: &mut TV) {
        tv.off();
        println!("Undo: TV turned off.");
    }
}
