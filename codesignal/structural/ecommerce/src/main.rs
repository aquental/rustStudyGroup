mod features;
mod payment;
mod product;

use features::{DiscountFeature, GiftWrapFeature};
use payment::{
    PayPal, PayPalAdapter, PaymentGateway, Square, SquareAdapter, Stripe, StripeAdapter,
};
use product::{Product, ProductBundle, ProductComponent};

fn main() {
    // Adapter Pattern Usage
    // TODO: Create instances of the following payment gateways:
    // - PayPal
    // - Stripe
    // - Square
    let pay_pal = PayPal {};
    let stripe = Stripe {};
    let square = Square {};

    // TODO: Create corresponding adapter instances for each payment gateway:
    // - PayPalAdapter
    // - StripeAdapter
    // - SquareAdapter
    let pay_pal_adapter = PayPalAdapter::new(pay_pal);
    let stripe_adapter = StripeAdapter::new(stripe);
    let square_adapter = SquareAdapter::new(square);

    // TODO: Process payments through each adapter:
    // - Process a payment of $50.00 through PayPalAdapter
    // - Process a payment of $75.00 through StripeAdapter
    // - Process a payment of $100.00 through SquareAdapter
    pay_pal_adapter.process_payment("50.00");
    stripe_adapter.process_payment("75.00");
    square_adapter.process_payment("100.00");

    // Composite and Decorator Pattern Usage
    // TODO: Create basic products:
    // - Product: "Phone" with price $699.00
    // - Product: "Laptop" with price $999.00
    let phone = Product::new("Phone".to_string(), 699.0);
    let laptop = Product::new("Laptop".to_string(), 999.0);

    // TODO: Decorate products:
    // - Wrap the phone in a DiscountFeature with $50.00 discount
    // - Wrap the laptop in a GiftWrapFeature
    let discounted_phone = DiscountFeature::new(Box::new(phone), 50.0);
    let gift_wrapped_laptop = GiftWrapFeature::new(Box::new(laptop));

    // TODO: Create a ProductBundle and add the decorated products:
    // - Add the discounted phone to the ProductBundle
    // - Add the gift-wrapped laptop to the ProductBundle
    let mut electronics_bundle = ProductBundle::new();
    electronics_bundle.add(Box::new(discounted_phone));
    electronics_bundle.add(Box::new(gift_wrapped_laptop));

    // TODO: Show details of the ProductBundle
    // - Print "\nElectronics Bundle Details:"
    // - Call electronics_bundle.show_details()
    println!("\nElectronics Bundle Details:");
    electronics_bundle.show_details();
}
