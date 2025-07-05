pub trait PaymentGateway {
    fn process_payment(&self, amount: &str);
}

// TODO: Define the PayPal struct
// TODO: Implement the make_payment method in PayPal
// Method should print "Processing payment of ${amount} through PayPal."
pub struct PayPal;
impl PayPal {
    fn make_payment(&self, amount: &str) {
        println!("Processing payment of ${amount} through PayPal.");
    }
}

// TODO: Define the Stripe struct
// TODO: Implement the handle_payment method in Stripe
// Method should print "Processing payment of ${amount} through Stripe."
pub struct Stripe;
impl Stripe {
    fn make_payment(&self, amount: &str) {
        println!("Processing payment of ${amount} through Stripe.");
    }
}

// TODO: Define the Square struct
// TODO: Implement the process_payment_square method in Square
// Method should print "Processing payment of ${amount} through Square."
pub struct Square;
impl Square {
    fn make_payment(&self, amount: &str) {
        println!("Processing payment of ${amount} through Square.");
    }
}

// TODO: Define the PayPalAdapter struct
// - Should have a private field 'pay_pal' of type PayPal
// TODO: Implement the PayPalAdapter constructor
// TODO: Implement the PaymentGateway trait for PayPalAdapter
//     Implement the process_payment method in PayPalAdapter
//     Method should call self.pay_pal.make_payment(amount)
pub struct PayPalAdapter {
    pay_pal: PayPal,
}
impl PayPalAdapter {
    pub fn new(pay_pal: PayPal) -> PayPalAdapter {
        PayPalAdapter { pay_pal }
    }
}
impl PaymentGateway for PayPalAdapter {
    fn process_payment(&self, amount: &str) {
        self.pay_pal.make_payment(amount);
    }
}

// TODO: Define the StripeAdapter struct
// - Should have a private field 'stripe' of type Stripe
// TODO: Implement the StripeAdapter constructor
// TODO: Implement the PaymentGateway trait for StripeAdapter
//     Implement the process_payment method in StripeAdapter
//     Method should call self.stripe.handle_payment(amount)
pub struct StripeAdapter {
    stripe: Stripe,
}
impl StripeAdapter {
    pub fn new(stripe: Stripe) -> StripeAdapter {
        StripeAdapter { stripe }
    }
}
impl PaymentGateway for StripeAdapter {
    fn process_payment(&self, amount: &str) {
        self.stripe.make_payment(amount);
    }
}

// TODO: Define the SquareAdapter struct
// - Should have a private field 'square' of type Square
// TODO: Implement the SquareAdapter constructor
// TODO: Implement the PaymentGateway trait for SquareAdapter
//     Implement the process_payment method in SquareAdapter
//     Method should call self.square.process_payment_square(amount)
pub struct SquareAdapter {
    square: Square,
}
impl SquareAdapter {
    pub fn new(square: Square) -> SquareAdapter {
        SquareAdapter { square }
    }
}
impl PaymentGateway for SquareAdapter {
    fn process_payment(&self, amount: &str) {
        self.square.make_payment(amount);
    }
}
