use crate::{command::Command, tv::TV};

// TODO: Define the RemoteControl struct
// - It should have a history of commands executed
// Implement the following methods:
// - new(): creates a new RemoteControl
// - press_button(command: Box<dyn Command>, tv: &mut TV): executes the command and adds it to history
// - undo_button(tv: &mut TV): undoes the last command from history
pub struct RemoteControl {
    history: Vec<Box<dyn Command>>,
    current_command: Option<Box<dyn Command>>,
}
impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl {
            history: Vec::new(),
            current_command: None,
        }
    }
    pub fn press_button(&mut self, mut command: Box<dyn Command>, tv: &mut TV) {
        command.execute(tv);
        self.history.push(command);
    }
    pub fn undo_button(&mut self, tv: &mut TV) {
        if let Some(mut command) = self.history.pop() {
            command.undo(tv);
        }
    }
}
