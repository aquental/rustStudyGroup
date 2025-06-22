/**
 Transition from using panic! to using Rust's Result type and the '?' operator for error propagation. The current codebase uses unchecked errors with panic!, leading to less robust and less readable code. Refactor the code by using Result, define meaningful error types, and use the '?' operator to handle errors cleanly, making the code more idiomatic and maintainable.
The application simulates a library system that manages book borrowing operations. The current implementation handles errors by panicking, but our goal is to refactor it to use the Result construct, custom error types, and proper error propagation to improve code quality and readability.
 **/
fn calculate_average(scores: &[f64]) -> f64 {
    let total: f64 = scores.iter().sum();
    total / scores.len() as f64
}

fn main() {
    let class_a_scores = [85.5, 90.0, 78.5, 88.0];
    let class_b_scores = [92.0, 81.5, 79.0, 85.5];

    let average_class_a = calculate_average(&class_a_scores);
    let average_class_b = calculate_average(&class_b_scores);

    println!("Average score for Class A: {}", average_class_a);
    println!("Average score for Class B: {}", average_class_b);
}
