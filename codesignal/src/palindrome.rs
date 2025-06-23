/**
Simplify string manipulations.
The current code is intended to check if a given string is a palindrome, but the logic is unnecessarily convoluted with redundant steps.
Refactor the code to make it more straightforward and easy to understand while preserving its functionality. Keep the code simple, concise, and readable by applying the concepts learned in this lesson.

Hint: Rust provides several useful string manipulation methods that can greatly simplify our code:

.chars(): converts a string into an iterator of characters
.rev(): reverses an iterator
.collect::<String>(): collects characters back into a String
.to_lowercase(): converts a string to lowercase
Make this palindrome checker as simple as possible!
**/
fn main() {
    let word = "radar";
    let palindrome = is_palindrome(word);
    println!("{} is a palindrome: {}", word, palindrome);
}

// Simplified palindrome checker using Rust string methods
fn is_palindrome(word: &str) -> bool {
    let normalized = word.to_lowercase();
    let reversed: String = normalized.chars().rev().collect();
    normalized == reversed
}
