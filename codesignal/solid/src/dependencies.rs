/**
Reduce dependencies with Ownership and Borrowing.
Improve dependencies.rs by refactoring it to reduce tight coupling and unnecessary cloning.
The current code excessively clones data and tightly couples structs by having them own instances directly. Your task is to refactor the code to borrow data where appropriate and decouple the structs to promote flexibility and maintainability.
The program models a simple notification system where a User can receive messages through a Notifier. Currently, the User owns the Notifier directly, leading to tight coupling and unnecessary data cloning. Leverage Rust's ownership and borrowing principles to clean up the code.
 */
struct Notifier {
    message: String,
}

impl Notifier {
    fn new(message: &str) -> Self {
        Notifier {
            message: message.to_string(),
        }
    }

    fn notify(&self, user_name: &str) {
        println!("{}: {}", user_name, self.message);
    }
}

struct User {
    name: String,
}

impl User {
    fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }

    fn receive_notification(&self, notifier: &Notifier) {
        notifier.notify(&self.name);
    }
}

fn main() {
    // Demonstrating proper ownership and borrowing
    let notifier = Notifier::new("You have a new message!");
    let user1 = User::new("Alice");
    let user2 = User::new("Bob");
    
    // Multiple users can now share the same notifier through borrowing
    user1.receive_notification(&notifier);
    user2.receive_notification(&notifier);
    
    // The notifier can still be used after being borrowed
    println!("Notifier is still available for reuse!");
}
