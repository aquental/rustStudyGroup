/**
 
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
