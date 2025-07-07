pub trait PaymentStrategy {
    fn pay(&self, amount: u32);
}

// TODO: Define the CreditCardStrategy struct that implements PaymentStrategy
// - Add a field for 'card_number' and implement the pay method to output:
// "Paid <amount> using Credit Card: <card_number>"
pub struct CreditCardStrategy {
    card_number: String,
}
impl CreditCardStrategy {
    pub fn new(card_number: String) -> CreditCardStrategy {
        CreditCardStrategy { card_number }
    }
}
impl PaymentStrategy for CreditCardStrategy {
    fn pay(&self, amount: u32) {
        println!("Paid {} using Credit Card: {}", amount, self.card_number);
    }
}

// TODO: Define the PayPalStrategy struct that implements PaymentStrategy
// - Add a field for 'email' and implement the pay method to output:
// "Paid <amount> using PayPal: <email>"
pub struct PayPalStrategy {
    email: String,
}
impl PayPalStrategy {
    pub fn new(email: String) -> PayPalStrategy {
        PayPalStrategy { email }
    }
}
impl PaymentStrategy for PayPalStrategy {
    fn pay(&self, amount: u32) {
        println!("Paid {} using PayPal: {}", amount, self.email);
    }
}

// TODO: Define the ShoppingCart struct that uses PaymentStrategy
// - Add a method set_payment_strategy that accepts a Box<dyn PaymentStrategy>
// - Add a method checkout that accepts an amount
//   - Executes the strategy's pay method if available
//   - Otherwise, it should output: "No payment strategy set."
pub struct ShoppingCart {
    payment_strategy: Option<Box<dyn PaymentStrategy>>,
}
impl ShoppingCart {
    pub fn new() -> ShoppingCart {
        ShoppingCart {
            payment_strategy: None,
        }
    }
    pub fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.payment_strategy = Some(strategy);
    }
    pub fn checkout(&self, amount: u32) {
        if let Some(ref strategy) = self.payment_strategy {
            strategy.pay(amount);
        } else {
            println!("No payment strategy set.");
        }
    }
}
