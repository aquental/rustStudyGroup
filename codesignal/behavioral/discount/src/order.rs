use crate::customer::Customer;
use crate::discount_strategy::DiscountStrategy;
use std::collections::HashMap;

// TODO: Create an Order struct
// - Constructor parameters: id (i32), price (f64)
// - Private field observers: HashMap<String, Customer>
// - Methods:
//   - apply_discount(strategy: Box<dyn DiscountStrategy>): void
//   - add_observer(customer: Customer): void
//   - notify_observers(status: &str): void
//   - place(): void  => notifies observers "Order placed with price: ${:.2}"
//   - cancel(): void => notifies observers "Order canceled"
//   - update_details(details: &str): void => notifies observers "Order updated: {details}"
pub struct Order {
    id: i32,
    price: f64,
    observers: HashMap<String, Customer>,
}
impl Order {
    pub fn new(id: i32, price: f64) -> Self {
        Order {
            id,
            price,
            observers: HashMap::new(),
        }
    }
    pub fn apply_discount(&mut self, strategy: Box<dyn DiscountStrategy>) {
        self.price = strategy.apply_discount(self.price);
    }
    pub fn add_observer(&mut self, customer: Customer) {
        let name = customer.name.clone();
        self.observers.insert(name, customer);
    }
    pub fn notify_observers(&self, status: &str) {
        for (_, customer) in &self.observers {
            customer.update(status);
        }
    }
    pub fn place(&self) {
        self.notify_observers(&format!("Order placed with price: ${:.2}", self.price));
    }
    pub fn cancel(&self) {
        self.notify_observers("Order canceled");
    }
    pub fn update_details(&self, details: &str) {
        self.notify_observers(&format!("Order updated: {}", details));
    }
    pub fn cancel_order(&self) {
        self.notify_observers("Order canceled");
    }
    pub fn update_order(&self, details: &str) {
        self.notify_observers(&format!("Order updated: {}", details));
    }
}
