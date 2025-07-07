pub trait PaymentStrategy {
    fn pay(&self, amount: u32);
}

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

pub struct ApplePayStrategy {
    phone_number: String,
}

impl ApplePayStrategy {
    pub fn new(phone_number: &str) -> Self {
        ApplePayStrategy {
            phone_number: phone_number.to_string(),
        }
    }
}

impl PaymentStrategy for ApplePayStrategy {
    fn pay(&self, amount: u32) {
        println!("Paid ${} using Apple Pay: {}", amount, self.phone_number);
    }
}

// TODO: Define the ShoppingCart struct
pub struct ShoppingCart {
    payment_strategy: Option<Box<dyn PaymentStrategy>>,
}

// TODO: Implement the ShoppingCart struct
// TODO: Implement the constructor
// - Returns a new ShoppingCart with no strategy set
impl ShoppingCart {
    pub fn new() -> Self {
        ShoppingCart {
            payment_strategy: None,
        }
    }

    // TODO: Define the set_payment_strategy method
    // - Takes a PaymentStrategy as an argument
    // - Sets the current strategy
    pub fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.payment_strategy = Some(strategy);
    }

    // TODO: Define the checkout method
    // - Takes an integer amount as argument
    // - Uses the current strategy to pay or prints "No payment strategy set." if no strategy is set
    pub fn checkout(&self, amount: u32) {
        if let Some(ref strategy) = self.payment_strategy {
            strategy.pay(amount);
        } else {
            println!("No payment strategy set.");
        }
    }
}
