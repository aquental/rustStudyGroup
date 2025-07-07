use std::vec;

use crate::payment::ShoppingCart;

mod payment;

fn main() {
    // TODO: Create an instance of the ShoppingCart struct
    let mut payment_strategies = ShoppingCart::new();

    // TODO: Create a vector of payment strategies including:
    // - CreditCardStrategy with number "1234-5678-9876-5432"
    // - PayPalStrategy with email "user@example.com" 
    // - ApplePayStrategy with phone "555-444-3333"
    let vec: Vec<Box<dyn payment::PaymentStrategy>> = vec![
        Box::new(payment::CreditCardStrategy::new("1234-5678-9876-5432")),
        Box::new(payment::PayPalStrategy::new("user@example.com")),
        Box::new(payment::ApplePayStrategy::new("555-444-3333")),
    ];

    for strategy in vec {
        // TODO: Set the payment strategy for the cart and checkout with amount 100
        payment_strategies.set_payment_strategy(strategy);
        payment_strategies.checkout(100);
    }
}
