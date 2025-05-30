fn main() {
    let box1: Box<i32> = Box::new(5);
    drop(box1); // First drop is fine
    //drop(box1); // ERROR: use of moved value `box1`
}
