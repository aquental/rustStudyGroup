enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("Quit message received."),
        Message::Move { x, y } => println!("Move to coordinates ({}, {}).", x, y),
        Message::Write(text) => println!("Message: {}", text),
    }
}

fn main() {
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("Hello, Rust!"));

    process_message(msg1);
    process_message(msg2);
}
