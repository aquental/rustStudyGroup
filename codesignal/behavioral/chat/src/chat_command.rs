use crate::chat_room::ChatRoom;

// TODO: Define the Command trait
// - Include a method execute
pub trait Command {
    fn execute(&self, chat_room: &mut ChatRoom);
}

// TODO: Define the ChatCommand struct implementing the Command trait
// - Define a constructor to initialize the message
// - Implement an execute method to show and broadcast the message
pub struct ChatCommand {
    message: String,
}
impl ChatCommand {
    pub fn new(message: String) -> ChatCommand {
        ChatCommand { message }
    }
}
impl Command for ChatCommand {
    fn execute(&self, chat_room: &mut ChatRoom) {
        chat_room.show_message(self.message.clone());
        chat_room.notify_observers(self.message.clone());
    }
}
