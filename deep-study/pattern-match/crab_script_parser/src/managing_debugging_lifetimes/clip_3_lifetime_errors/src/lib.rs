// CrabScript parser with lifetime error debugging
pub fn get_slice(input: &str) -> &str {
    // Objective: Fix 'does not live long enough' error
    // Task: Adjust scope to extend lifetime
    // Hint: Take input as parameter instead of local

    &input[1..]


}