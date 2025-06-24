/**
Improve the simplicity and readability of the current code.
It calculates the total cost of an order, but it contains some unnecessary complexity that can be removed.
Refactor the code to make it more straightforward and maintainable without altering its functionality.
Keep in mind the strategies you learned about applying the KISS principle during your lesson.
The goal is to reduce complexity and increase clarity in the code. Let’s make it simple and easy to read!
**/
fn main() {
    let item_price = 12.50;
    let quantity = 4;
    let discount_rate = 0.10;
    let tax_rate = 0.08;

    let total_cost = calculate_total_cost(item_price, quantity, discount_rate, tax_rate);
    println!("Total Cost: {:.2}", total_cost);
}

fn calculate_total_cost(item_price: f64, quantity: i32, discount_rate: f64, tax_rate: f64) -> f64 {
    let subtotal = item_price * quantity as f64;
    let discounted_price = subtotal * (1.0 - discount_rate);
    discounted_price * (1.0 + tax_rate)
}
