fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    let first: &i32 = &v[0];  // Immutable borrow
    v.clear(); // ERROR: cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{}", first); // Invalid use of the reference after `clear()`
}
