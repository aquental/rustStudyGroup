fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    let reference = &numbers[0]; // Immutable borrow
    numbers.clear(); // ERROR: cannot borrow `numbers` as mutable because it is also borrowed as immutable
    println!("First number: {}", reference);
}
