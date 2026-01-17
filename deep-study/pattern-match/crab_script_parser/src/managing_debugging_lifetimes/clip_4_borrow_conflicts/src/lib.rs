// CrabScript parser with borrow conflict resolution
pub fn modify_data(data: &mut Vec<i32>) {
    // Objective: Resolve mutable borrow conflicts
    // Task: Limit scope of borrows to allow mutation
    // Hint: Use block scope
    
    {
        let borrow_mut = data;
        borrow_mut.push(5); // Now allowed
        println!("Modified inside block: {:?}", borrow_mut);
    } // borrow_mut goes out of scope here

}