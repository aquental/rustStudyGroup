// Takes ownership of a String and prints it
fn takes_ownership(s: String) {
    println!("{}", s);
}

// Borrows a String reference and returns its length
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutably borrows a String and appends ", world!" to it
fn change(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    // Ownership example
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid here as ownership was moved
    
    // Borrowing example
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable reference example
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);  // Prints "hello, world!"
}
