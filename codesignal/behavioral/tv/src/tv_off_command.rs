use crate::{command::Command, tv::TV};

// TODO: Define the TVOffCommand struct and implement Command for it
// - The execute method should turn the TV off
// - The undo method should turn the TV on and print "Undo: TV turned on."
pub struct TVOffCommand {
    tv: TV,
}
impl TVOffCommand {
    pub fn new(tv: TV) -> Self {
        TVOffCommand { tv }
    }
}
impl Command for TVOffCommand {
    fn execute(&mut self, tv: &mut TV) {
        tv.off();
    }
    fn undo(&mut self, tv: &mut TV) {
        tv.on();
        println!("Undo: TV turned on.");
    }
}
