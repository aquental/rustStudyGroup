use crate::tv::TV;

// TODO: Define the Command trait
// - Method: fn execute(&mut self, tv: &mut TV);
// - Method: fn undo(&mut self, tv: &mut TV);
pub trait Command {
    fn execute(&mut self, tv: &mut TV);
    fn undo(&mut self, tv: &mut TV);
}
