mod payment_strategy;

use payment_strategy::{
    amazon_pay_strategy, credit_card_strategy::CreditCardStrategy, paypal_strategy::PayPalStrategy,
    shopping_cart::ShoppingCart,
};

fn main() {
    let mut cart = ShoppingCart::new();

    // Create payment strategies
    let credit_card = CreditCardStrategy::new("1234-5678-9876-5432");
    let pay_pal = PayPalStrategy::new("user@example.com");

    // TODO: Create an AmazonPayStrategy object with the email "amazon@user.com"
    let amazon_pay = amazon_pay_strategy::AmazonPayStrategy::new("amazon@user.com");

    // Use CreditCard strategy and checkout
    cart.set_payment_strategy(Box::new(credit_card));
    cart.checkout(100);

    // Switch to PayPal strategy and checkout
    cart.set_payment_strategy(Box::new(pay_pal));
    cart.checkout(150);

    // TODO: Set the payment strategy to amazon_pay and call the checkout method with an amount of 200
    cart.set_payment_strategy(Box::new(amazon_pay));
    cart.checkout(200);
}
