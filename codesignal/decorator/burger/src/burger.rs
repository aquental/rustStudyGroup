// The Burger trait defines the interface for all burgers
pub trait Burger {
    fn get_description(&self) -> String;
    fn cost(&self) -> f64;
}

// Concrete Component: The base burger without any additions
pub struct SimpleBurger;

impl Burger for SimpleBurger {
    fn get_description(&self) -> String {
        "Burger".to_string()
    }

    fn cost(&self) -> f64 {
        5.0
    }
}

// TODO: Implement CheeseDecorator struct:
// - Implement the Burger trait
// - Add ", Cheese" to the burger's description
// - Increase the cost by 1.0
pub struct CheeseDecorator {
    pub burger: Box<dyn Burger>,
}
impl Burger for CheeseDecorator {
    fn get_description(&self) -> String {
        self.burger.get_description() + ", Cheese"
    }
    fn cost(&self) -> f64 {
        self.burger.cost() + 1.0
    }
}

// TODO: Implement BaconDecorator struct:
// - Implement the Burger trait
// - Add ", Bacon" to the burger's description
// - Increase the cost by 1.5
pub struct BaconDecorator {
    pub burger: Box<dyn Burger>,
}
impl Burger for BaconDecorator {
    fn get_description(&self) -> String {
        self.burger.get_description() + ", Bacon"
    }
    fn cost(&self) -> f64 {
        self.burger.cost() + 1.5
    }
}
