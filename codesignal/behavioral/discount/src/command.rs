use crate::order::Order;

// TODO: Define a trait Command
// - Include a method execute(&self, order: &mut Order)
pub trait Command {
    fn execute(&self, order: &mut Order);
}

// TODO: Create a struct PlaceOrderCommand implementing Command
// - Constructor: new() -> Self
// - Implement execute(&self, order: &mut Order)
//   - Calls place() on the Order object
pub struct PlaceOrderCommand;
impl PlaceOrderCommand {
    pub fn new() -> Self {
        PlaceOrderCommand
    }
}
impl Command for PlaceOrderCommand {
    fn execute(&self, order: &mut Order) {
        order.place();
    }
}

// TODO: Create a struct CancelOrderCommand implementing Command
// - Constructor: new() -> Self
// - Implement execute(&self, order: &mut Order)
//   - Calls cancel() on the Order object
pub struct CancelOrderCommand;
impl CancelOrderCommand {
    pub fn new() -> Self {
        CancelOrderCommand
    }
}
impl Command for CancelOrderCommand {
    fn execute(&self, order: &mut Order) {
        order.cancel();
    }
}

// TODO: Create a struct UpdateOrderCommand implementing Command
// - Constructor: new(details: String) -> Self
// - Implement execute(&self, order: &mut Order)
//   - Calls update_details() on the Order object with the provided details
pub struct UpdateOrderCommand {
    details: String,
}
impl UpdateOrderCommand {
    pub fn new(details: String) -> Self {
        UpdateOrderCommand { details }
    }
}
impl Command for UpdateOrderCommand {
    fn execute(&self, order: &mut Order) {
        order.update_details(&self.details);
    }
}
