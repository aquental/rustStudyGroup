use crate::{command::Command, fan::Fan};

// TODO: Implement the RemoteControl struct that executes and undoes commands
// - Define a struct named `RemoteControl`.
// - Inside `RemoteControl`, maintain a history of commands.
// - Add a method `press_button(command: Box<dyn Command>, fan: &mut Fan)` that executes the command and stores it in history.
// - Add a method `undo_button(fan: &mut Fan)` that undoes the last executed command.

pub struct RemoteControl {
    history: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    pub fn press_button(&mut self, command: Box<dyn Command>, fan: &mut Fan) {
        command.execute(fan);
        self.history.push(command);
    }

    pub fn undo_button(&mut self, fan: &mut Fan) {
        if let Some(command) = self.history.pop() {
            command.undo(fan);
        }
    }
}
