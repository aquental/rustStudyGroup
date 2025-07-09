use crate::device::Device;

// TODO: Create a trait Command
// - Define execute method that takes &mut dyn Device
pub trait Command {
    fn execute(&self, device: &mut dyn Device);
}
