mod pizza;
mod pizza_decorator;
use crate::pizza_decorator::{CheeseDecorator, PepperoniDecorator, OlivesDecorator};
use pizza::Pizza;

fn main() {
    // TODO: Create a new PlainPizza object.
    // TODO: Print the description and cost.
    let plain_pizza = Box::new(pizza::PlainPizza::new());
    println!("Description: {}", plain_pizza.description());
    println!("Cost: {}", plain_pizza.cost());

    // TODO: Wrap the pizza object with CheeseDecorator.
    // TODO: Print the updated description and cost.
    let cheese_pizza = Box::new(CheeseDecorator { pizza: plain_pizza });
    println!("Description: {}", cheese_pizza.description());
    println!("Cost: {}", cheese_pizza.cost());

    // TODO: Wrap the pizza object with PepperoniDecorator.
    // TODO: Print the updated description and cost.
    let pepperoni_pizza = Box::new(PepperoniDecorator {
        pizza: cheese_pizza,
    });
    println!("Description: {}", pepperoni_pizza.description());
    println!("Cost: {}", pepperoni_pizza.cost());

    // TODO: Wrap the pizza object with OlivesDecorator.
    // TODO: Print the updated description and cost.
    let olives_pizza = Box::new(OlivesDecorator {
        pizza: pepperoni_pizza,
    });
    println!("Description: {}", olives_pizza.description());
    println!("Cost: {}", olives_pizza.cost());
}
