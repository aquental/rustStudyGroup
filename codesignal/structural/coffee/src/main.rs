mod coffee;
mod coffee_decorator;
mod coffee_set;

use coffee::*;
use coffee_decorator::*;
use coffee_set::*;

fn main() {
    // TODO: Create an instance of SimpleCoffee
    let simple_coffee = SimpleCoffee::new();

    // TODO: Create an instance of VanillaDecorator by wrapping the SimpleCoffee instance
    let vanilla_coffee = VanillaDecorator::new(Box::new(simple_coffee));

    // TODO: Create an instance of MilkDecorator by wrapping the VanillaDecorator instance
    let milk_coffee = MilkDecorator::new(Box::new(vanilla_coffee));

    // TODO: Create an instance of CoffeeSet
    let mut coffee_set = CoffeeSet::new();

    // TODO: Add the SimpleCoffee, VanillaDecorator, and MilkDecorator instances to the CoffeeSet
    coffee_set.add(Box::new(SimpleCoffee::new()));
    coffee_set.add(Box::new(VanillaDecorator::new(Box::new(SimpleCoffee::new()))));
    coffee_set.add(Box::new(milk_coffee));

    // TODO: Print the description of the CoffeeSet
    println!("{}", coffee_set.get_description());

    // TODO: Print the total cost of the CoffeeSet
    println!("Total cost: {}", coffee_set.cost());
}
