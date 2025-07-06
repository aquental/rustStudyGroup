use crate::{command::Command, tv::TV};

// TODO: Define the RemoteControl struct
// - It should have a history of commands executed
// Implement the following methods:
// - new(): creates a new RemoteControl
// - press_button(command: Box<dyn Command>, tv: &mut TV): executes the command and adds it to history
// - undo_button(tv: &mut TV): undoes the last command from history
