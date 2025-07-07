mod payment_strategy;

use payment_strategy::{CreditCardStrategy, PayPalStrategy, ShoppingCart};

fn main() {
    let mut cart = ShoppingCart::new();

    // Create payment strategies
    let credit_card = CreditCardStrategy::new("1234-5678-9876-5432".to_string());
    let pay_pal = PayPalStrategy::new("user@example.com".to_string());

    // Use CreditCard strategy and checkout
    cart.set_payment_strategy(Box::new(credit_card));
    cart.checkout(100);

    // Switch to PayPal strategy and checkout
    cart.set_payment_strategy(Box::new(pay_pal));
    cart.checkout(200);
}
