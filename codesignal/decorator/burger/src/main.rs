mod burger;
use burger::{Burger, SimpleBurger, CheeseDecorator, BaconDecorator};

fn main() {
    let my_burger = Box::new(SimpleBurger);
    println!("{} ${:.2}", my_burger.get_description(), my_burger.cost());

    // TODO: Add cheese to the burger using CheeseDecorator
    // Print the description and cost of your burger with cheese
    let cheese_burger = Box::new(CheeseDecorator { burger: my_burger });
    println!("{} ${:.2}", cheese_burger.get_description(), cheese_burger.cost());

    // TODO: Add bacon to the burger using BaconDecorator
    // Print the description and cost of your burger with cheese and bacon
    let bacon_cheese_burger = Box::new(BaconDecorator { burger: cheese_burger });
    println!("{} ${:.2}", bacon_cheese_burger.get_description(), bacon_cheese_burger.cost());
}
