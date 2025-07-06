use crate::command::Command;
use crate::fan::Fan;

// TODO: Implement a RemoteControl struct that sets and executes commands.
// - Create a struct named RemoteControl with a field `history` that is a Vec<Box<dyn Command>>.
// - Implement a constructor `new` for RemoteControl.
// - Implement a method `press_button` that takes a `Box<dyn Command>` and a mutable reference to `Fan`.
//     - The method should execute the command and add it to the history.
// - Implement a method `undo_button` that takes a mutable reference to `Fan`.
//     - The method should pop the last command from history and undo it.
//     - If there is no command to undo, it should print "No commands to undo.".

pub struct RemoteControl {
    history: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    pub fn new() -> Self {
        RemoteControl {
            history: Vec::new(),
        }
    }
    pub fn press_button(&mut self, mut command: Box<dyn Command>, fan: &mut Fan) {
        command.execute(fan);
        self.history.push(command);
    }
    pub fn undo_button(&mut self, fan: &mut Fan) {
        if let Some(mut command) = self.history.pop() {
            command.undo(fan);
        } else {
            println!("No commands to undo.");
        }
    }
}
