use crate::payment_strategy::payment_strategy_trait::PaymentStrategy;

// TODO: Define the AmazonPayStrategy struct below that implements PaymentStrategy
// - Create a constructor that takes an email address as a parameter
// - Initialize the email member variable
// - Implement the pay method
//   - Print the payment amount and the email address as follows:
//   "Paid <amount> using Amazon Pay: <email>"
pub struct AmazonPayStrategy {
    email: String,
}

impl AmazonPayStrategy {
    pub fn new(email: &str) -> Self {
        AmazonPayStrategy {
            email: email.to_string(),
        }
    }
}

impl PaymentStrategy for AmazonPayStrategy {
    fn pay(&self, amount: u32) {
        println!("Paid ${} using Amazon Pay: {}", amount, self.email);
    }
}
