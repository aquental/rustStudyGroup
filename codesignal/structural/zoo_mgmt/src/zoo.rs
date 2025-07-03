// TODO: Define a trait `AnimalComponent` with a method `display_details()`
pub trait AnimalComponent {
    fn display_details(&self);
}

// TODO: Implement a struct `Animal` that implements `AnimalComponent`
// - Attributes: `name`, `species`
// - Implement `display_details()` method to display: "Animal: <name> (<species>)"
pub struct Animal {
    name: String,
    species: String,
}
impl Animal {
    pub fn new(name: &str, species: &str) -> Self {
        Self {
            name: name.to_string(),
            species: species.to_string(),
        }
    }
}
impl AnimalComponent for Animal {
    fn display_details(&self) {
        println!("Animal: {} ({})", self.name, self.species);
    }
}

// TODO: Implement a struct `Zoo` that implements `AnimalComponent`
// - Create a Vec to manage `AnimalComponent` objects
// - Provide a method to add a `Box<dyn AnimalComponent>` to the zoo
// - Implement `display_details()` method to display all components
pub struct Zoo {
    animals: Vec<Box<dyn AnimalComponent>>,
}
impl Zoo {
    pub fn new() -> Self {
        Self { animals: vec![] }
    }
    pub fn add_animal(&mut self, animal: Box<dyn AnimalComponent>) {
        self.animals.push(animal);
    }
}

impl AnimalComponent for Zoo {
    fn display_details(&self) {
        println!("Zoo: ");
        for animal in &self.animals {
            animal.display_details();
        }
    }
}
