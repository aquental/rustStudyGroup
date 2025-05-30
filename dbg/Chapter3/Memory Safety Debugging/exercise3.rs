fn main() {
    let reference: &i32;
    {
        let value: i32 = 42;
        reference = &value; // Borrowing `value`
    } // `value` goes out of scope here
   // println!("The answer is: {}", reference); //  ERROR: `value` does not live long enough
}
