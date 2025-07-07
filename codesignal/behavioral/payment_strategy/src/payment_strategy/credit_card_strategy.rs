use crate::payment_strategy::payment_strategy_trait::PaymentStrategy;

pub struct CreditCardStrategy {
    card_number: String,
}

impl CreditCardStrategy {
    pub fn new(card_number: &str) -> Self {
        CreditCardStrategy {
            card_number: card_number.to_string(),
        }
    }
}

impl PaymentStrategy for CreditCardStrategy {
    fn pay(&self, amount: u32) {
        println!("Paid ${} using Credit Card: {}", amount, self.card_number);
    }
}
