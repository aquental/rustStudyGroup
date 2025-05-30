fn process_data(values: Vec<i32>) {
    println!("Processing {} items", values.len());
}
fn main() {
    let data: Vec<i32> = vec![1, 2, 3];
    process_data(data); // Moves ownership
    //println!("Data length: {}", data.len()); // ❌ ERROR: `data` has been moved
}
