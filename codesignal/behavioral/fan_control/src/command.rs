use crate::fan::Fan;

// TODO: Define the Command trait with execute and undo methods
// - Create a trait named `Command`.
// - Add method declarations `fn execute(&self, fan: &mut Fan)` and `fn undo(&self, fan: &mut Fan)` to the `Command` trait.

pub trait Command {
    fn execute(&self, fan: &mut Fan);
    fn undo(&self, fan: &mut Fan);
}
