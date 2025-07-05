use crate::coffee::Coffee;

// TODO: Define the AbstractCoffeeDecorator struct that implements Coffee
// - Add a field coffee of type Box<dyn Coffee>
// - Create a constructor function new that initializes the coffee field
struct AbstractCoffeeDecorator {
    coffee: Box<dyn Coffee>,
}
impl AbstractCoffeeDecorator {
    pub fn new(coffee: Box<dyn Coffee>) -> AbstractCoffeeDecorator {
        AbstractCoffeeDecorator { coffee }
    }
}
// TODO: Define the VanillaDecorator struct extending CoffeeDecorator
// - Implement get_description and append ", Vanilla" to the description of the wrapped Coffee object
// - Implement cost to add 0.5 to the cost of the wrapped Coffee object
pub struct VanillaDecorator {
    coffee: Box<dyn Coffee>,
}
impl VanillaDecorator {
    pub fn new(coffee: Box<dyn Coffee>) -> VanillaDecorator {
        VanillaDecorator { coffee }
    }
}
impl Coffee for VanillaDecorator {
    fn get_description(&self) -> String {
        self.coffee.get_description() + ", Vanilla"
    }
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5
    }
}

// TODO: Define the MilkDecorator struct extending CoffeeDecorator
// - Implement get_description and append ", Milk" to the description of the wrapped Coffee object
// - Implement cost to add 0.3 to the cost of the wrapped Coffee object
pub struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}
impl MilkDecorator {
    pub fn new(coffee: Box<dyn Coffee>) -> MilkDecorator {
        MilkDecorator { coffee }
    }
}
impl Coffee for MilkDecorator {
    fn get_description(&self) -> String {
        self.coffee.get_description() + ", Milk"
    }
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.3
    }
}
