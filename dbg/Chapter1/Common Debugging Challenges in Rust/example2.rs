fn main() {
    let mut original = String::from("Hello");
    {
        let borrowed = &original;  // Borrowing in separate scope
        println!("{}", borrowed);  // Prints: Hello
    }
    original.push_str(" World");  // Now this works!
    println!("{}", original);  // Prints: Hello World
}
