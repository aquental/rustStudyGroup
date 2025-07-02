mod customer;
mod customer_builder;
mod logger;

use customer_builder::{CustomerBuilder, PremiumCustomerBuilder, RegularCustomerBuilder};
use logger::Logger;

fn main() {
    let logger = Logger;
    
    // Build Premium Customer
    let mut premium_builder = PremiumCustomerBuilder::new();
    let premium_customer = premium_builder
        .build_name("Alice Smith".to_string())
        .build_email("alice@example.com".to_string())
        .build_extra_details("Gold");
    let premium_customer = premium_builder.build();
    premium_customer.show_info(&logger);

    // Build Regular Customer
    let mut regular_builder = RegularCustomerBuilder::new();
    let regular_customer = regular_builder
        .build_name("Bob Jones".to_string())
        .build_email("bob@example.com".to_string())
        .build_extra_details("true");
    let regular_customer = regular_builder.build();
    regular_customer.show_info(&logger);
}
