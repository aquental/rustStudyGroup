/**
Alternative implementation using enum-based polymorphism for static dispatch.
This approach provides better performance as method calls are resolved at compile time,
but is less extensible than the trait-based approach.
*/

// Define enum for different animal types
#[derive(Debug, Clone)]
enum Animal {
    Dog,
    Cat,
    Bird,
}

impl Animal {
    // Method for feeding behavior
    fn feed(&self) {
        match self {
            Animal::Dog => println!("Feeding the dog with dog food."),
            Animal::Cat => println!("Feeding the cat with cat food."),
            Animal::Bird => println!("Feeding the bird with bird seeds."),
        }
    }

    // Method for playing behavior
    fn play(&self) {
        match self {
            Animal::Dog => println!("Playing fetch with the dog."),
            Animal::Cat => println!("Playing with the cat using a ball of yarn."),
            Animal::Bird => println!("The bird is chirping happily but doesn't play."),
        }
    }

    // Method to get animal name
    fn name(&self) -> &'static str {
        match self {
            Animal::Dog => "Dog",
            Animal::Cat => "Cat",
            Animal::Bird => "Bird",
        }
    }

    // Combined interaction method
    fn interact(&self) {
        println!("Interacting with {}", self.name());
        self.feed();
        self.play();
        println!();
    }
}

// Helper functions for specific behaviors
fn feed_animal(animal: &Animal) {
    animal.feed();
}

fn play_with_animal(animal: &Animal) {
    animal.play();
}

fn interact_with_animal(animal: &Animal) {
    animal.interact();
}

fn main() {
    // Create a collection of animals
    let animals = vec![Animal::Dog, Animal::Cat, Animal::Bird];

    // Demonstrate individual behaviors
    println!("=== Individual Behaviors ===");
    for animal in &animals {
        feed_animal(animal);
        play_with_animal(animal);
        println!();
    }

    // Demonstrate combined behavior
    println!("=== Complete Interactions ===");
    for animal in &animals {
        interact_with_animal(animal);
    }

    // Demonstrate method chaining and filtering
    println!("=== Feeding Time ===");
    animals.iter().for_each(|animal| animal.feed());

    // Demonstrate pattern matching with additional logic
    println!("\n=== Special Behaviors ===");
    for animal in &animals {
        match animal {
            Animal::Dog => {
                println!("Dogs are loyal companions!");
                animal.feed();
                animal.play();
            }
            Animal::Cat => {
                println!("Cats are independent creatures!");
                animal.feed();
                animal.play();
            }
            Animal::Bird => {
                println!("Birds bring joy with their songs!");
                animal.feed();
                // Birds might not always want to play
                animal.play();
            }
        }
        println!();
    }
}
