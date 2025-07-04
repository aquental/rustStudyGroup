mod beverage;

use beverage::{Beverage, Espresso};

fn main() {
    let espresso = Box::new(Espresso);
    println!("{}: ${:.2}", espresso.description(), espresso.cost());

    // TODO: Decorate with Cinnamon and print the description and cost
    let cinnamon = Box::new(beverage::Cinnamon { coffee: espresso });
    println!("{}: ${:.2}", cinnamon.description(), cinnamon.cost());
}
