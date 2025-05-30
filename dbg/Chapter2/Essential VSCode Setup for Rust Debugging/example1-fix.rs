fn main() {
    let mut data = vec![1, 2, 3];
    data.push(4);         //  Mutate before borrowing
    let borrowed = &data;  //  Immutable borrow starts after mutation
    println!("Borrowed: {:?}", borrowed);  // Borrow still in use (valid)
}
