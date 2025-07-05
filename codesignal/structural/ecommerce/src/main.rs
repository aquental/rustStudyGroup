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

    // TODO: Create corresponding adapter instances for each payment gateway:
    // - PayPalAdapter
    // - StripeAdapter
    // - SquareAdapter

    // TODO: Process payments through each adapter:
    // - Process a payment of $50.00 through PayPalAdapter
    // - Process a payment of $75.00 through StripeAdapter
    // - Process a payment of $100.00 through SquareAdapter

    // Composite and Decorator Pattern Usage

    // TODO: Create basic products:
    // - Product: "Phone" with price $699.00
    // - Product: "Laptop" with price $999.00

    // TODO: Decorate products:
    // - Wrap the phone in a DiscountFeature with $50.00 discount
    // - Wrap the laptop in a GiftWrapFeature

    // TODO: Create a ProductBundle and add the decorated products:
    // - Add the discounted phone to the ProductBundle
    // - Add the gift-wrapped laptop to the ProductBundle

    // TODO: Show details of the ProductBundle
    // - Print "\nElectronics Bundle Details:"
    // - Call electronics_bundle.show_details()
}
