use crate::payment_strategy::payment_strategy_trait::PaymentStrategy;

pub struct PayPalStrategy {
    email: String,
}

impl PayPalStrategy {
    pub fn new(email: &str) -> Self {
        PayPalStrategy {
            email: email.to_string(),
        }
    }
}

impl PaymentStrategy for PayPalStrategy {
    fn pay(&self, amount: u32) {
        println!("Paid ${} using PayPal: {}", amount, self.email);
    }
}
