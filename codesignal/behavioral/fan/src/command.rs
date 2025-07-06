use crate::fan::Fan;

// TODO: Define a Command trait with execute and undo methods that accept a mutable reference to Fan.
// - Create a trait named Command.
// - Add method declarations for `execute` and `undo`, both accepting `&mut Fan` as a parameter.

pub trait Command {
    fn execute(&mut self, fan: &mut Fan);
    fn undo(&mut self, fan: &mut Fan);
}
