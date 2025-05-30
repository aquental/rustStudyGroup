fn main() {
    let mut data = vec![1, 2, 3];
    let borrowed = &data;  // Immutable borrow starts
    data.push(4);         // Attempt to mutate while borrowed
    println!("Borrowed: {:?}", borrowed);  // Borrow still in use
}
