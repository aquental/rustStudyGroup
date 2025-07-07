use crate::payment_strategy::payment_strategy_trait::PaymentStrategy;

pub struct ShoppingCart {
    strategy: Option<Box<dyn PaymentStrategy>>,
}

impl ShoppingCart {
    pub fn new() -> Self {
        ShoppingCart { strategy: None }
    }

    pub fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = Some(strategy);
    }

    pub fn checkout(&self, amount: u32) {
        match &self.strategy {
            Some(strategy) => strategy.pay(amount),
            None => println!("No payment strategy set."),
        }
    }
}
