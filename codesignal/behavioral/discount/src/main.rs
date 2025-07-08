mod command;
mod customer;
mod discount_strategy;
mod order;

use command::{CancelOrderCommand, Command, PlaceOrderCommand, UpdateOrderCommand};
use customer::Customer;
use discount_strategy::{DiscountStrategy, PercentageDiscount};
use order::Order;

fn main() {
    // TODO: Create instances of Customers "Alice" and "Manager"
    let alice = Customer::new("Alice".to_string());
    let manager = Customer::new("Manager".to_string());

    // TODO: Create a mutable Order with an ID of 1 and a price of 100.0
    let mut order = Order::new(1, 100.0);

    // TODO: Add both customers as observers to the order
    order.add_observer(alice);
    order.add_observer(manager);

    // TODO: Apply a discount to the order
    let strategy = PercentageDiscount::new(10.0);
    order.apply_discount(Box::new(strategy));

    // TODO: Place the order using PlaceOrderCommand
    //       call command.execute(&mut order)
    let command = PlaceOrderCommand::new();
    command.execute(&mut order);

    // TODO: Update the order using UpdateOrderCommand with "Shipping address updated"
    //       call command.execute(&mut order)
    let command = UpdateOrderCommand::new("Shipping address updated".to_string());
    command.execute(&mut order);

    // TODO: Cancel the order using CancelOrderCommand
    //       call command.execute(&mut order)
    let command = CancelOrderCommand::new();
    command.execute(&mut order);
}
