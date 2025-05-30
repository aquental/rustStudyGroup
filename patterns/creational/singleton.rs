//! ructc 1.63
//! https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//!
//! Starting with Rust 1.63, it can be easier to work with global mutable
//! singletons, although it's still preferable to avoid global variables in most
//! cases.
//!
//! Now that `Mutex::new` is `const`, you can use global static `Mutex` locks
//! without needing lazy initialization.
use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

/// Pushes the value 1 onto the global mutable singleton vector.
fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

/// Shows how to use a global mutable singleton.
///
/// The `do_a_call` calls add elements to the singleton vector, and we print
/// out the contents of the vector after locking it. Then we drop the lock so
/// that we can lock it again to print out the contents one more time.
///
/// The third lock and print shows that the lock is re-entrant, so that the same
/// thread can lock the mutex multiple times.
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    let array = ARRAY.lock().unwrap();
    println!("Called {} times: {:?}", array.len(), array);
    drop(array);

    *ARRAY.lock().unwrap() = vec![3, 4, 5];

    println!("New singleton object: {:?}", ARRAY.lock().unwrap());
}
