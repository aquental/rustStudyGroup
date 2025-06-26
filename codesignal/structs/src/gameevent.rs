// TODO: Define the `GameEvent` enum with three variants:
// `Exit` with no data;
// `PlayerMove` with named integer fields `x` and `y`;
// `ChatMessage` with a single String.
enum GameEvent {
    Exit,
    PlayerMove { x: i32, y: i32 },
    ChatMessage(String),
}

fn process_event(event: GameEvent) {
    match event {
        GameEvent::Exit => println!("Game exit requested."),
        GameEvent::PlayerMove { x, y } => println!("Player moved to position ({}, {}).", x, y),
        GameEvent::ChatMessage(text) => println!("New chat message: {}", text),
    }
}

fn main() {
    let event1 = GameEvent::PlayerMove { x: 10, y: 20 };
    let event2 = GameEvent::ChatMessage(String::from("Hello, players!"));

    process_event(event1);
    process_event(event2);
}
