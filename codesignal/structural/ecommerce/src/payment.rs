pub trait PaymentGateway {
    fn process_payment(&self, amount: &str);
}

// TODO: Define the PayPal struct
// TODO: Implement the make_payment method in PayPal
// Method should print "Processing payment of ${amount} through PayPal."

// TODO: Define the Stripe struct
// TODO: Implement the handle_payment method in Stripe
// Method should print "Processing payment of ${amount} through Stripe."

// TODO: Define the Square struct
// TODO: Implement the process_payment_square method in Square
// Method should print "Processing payment of ${amount} through Square."

// TODO: Define the PayPalAdapter struct
// - Should have a private field 'pay_pal' of type PayPal
// TODO: Implement the PayPalAdapter constructor
// TODO: Implement the PaymentGateway trait for PayPalAdapter
//     Implement the process_payment method in PayPalAdapter
//     Method should call self.pay_pal.make_payment(amount)

// TODO: Define the StripeAdapter struct
// - Should have a private field 'stripe' of type Stripe
// TODO: Implement the StripeAdapter constructor
// TODO: Implement the PaymentGateway trait for StripeAdapter
//     Implement the process_payment method in StripeAdapter
//     Method should call self.stripe.handle_payment(amount)

// TODO: Define the SquareAdapter struct
// - Should have a private field 'square' of type Square
// TODO: Implement the SquareAdapter constructor
// TODO: Implement the PaymentGateway trait for SquareAdapter
//     Implement the process_payment method in SquareAdapter
//     Method should call self.square.process_payment_square(amount)
