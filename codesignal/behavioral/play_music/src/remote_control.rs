use crate::command::Command;

pub struct RemoteControl {
    command: Option<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl { command: None }
    }

    pub fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = Some(command);
    }

    pub fn press_button(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("No command set.");
        }
    }
}
