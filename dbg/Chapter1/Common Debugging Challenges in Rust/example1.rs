fn main() {
    let mut original = String::from("Hello");
    let borrowed = &original;  
    original.push_str(" World");  //  Error: can't modify while borrowed
    println!("{}", borrowed);
}
