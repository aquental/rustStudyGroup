mod chat_command;
mod chat_room;
mod user;

use chat_command::{ChatCommand, Command};
use chat_room::ChatRoom;
use user::User;

fn main() {
    // TODO: Create a ChatRoom instance
    let mut chat = ChatRoom::new();

    // TODO: Create two User instances with names "Alice" and "Bob"
    let alice = User::new("Alice".to_string());
    let bob = User::new("Bob".to_string());

    // TODO: Add users to the chat room
    chat.add_observer(alice);
    chat.add_observer(bob);

    // TODO: Create a ChatCommand instance with a message "Hello, everyone!"
    let command = ChatCommand::new("Hello, everyone!".to_string());

    // TODO: Execute the command to display and broadcast the message
    command.execute(&mut chat);
}
