/**
Refactored code using trait-based polymorphism for animal behaviors.
This approach eliminates runtime type checking and uses dynamic dispatch through traits,
making the code more idiomatic, extensible, and maintainable.
Each animal implements the required behaviors, and trait objects enable polymorphic collections.
*/

// Define traits for different animal behaviors
trait Feedable {
    fn feed(&self);
}

trait Playable {
    fn play(&self);
}

// Combined trait for animals that can both be fed and played with
trait Animal: Feedable + Playable {
    fn name(&self) -> &'static str;
}

// Concrete animal types
struct Dog;
struct Cat;
struct Bird;

// Implement Feedable for each animal type
impl Feedable for Dog {
    fn feed(&self) {
        println!("Feeding the dog with dog food.");
    }
}

impl Feedable for Cat {
    fn feed(&self) {
        println!("Feeding the cat with cat food.");
    }
}

impl Feedable for Bird {
    fn feed(&self) {
        println!("Feeding the bird with bird seeds.");
    }
}

// Implement Playable for each animal type
// Note: Not all animals may be playable (e.g., Bird doesn't implement Playable)
impl Playable for Dog {
    fn play(&self) {
        println!("Playing fetch with the dog.");
    }
}

impl Playable for Cat {
    fn play(&self) {
        println!("Playing with the cat using a ball of yarn.");
    }
}

// Provide a default implementation for Bird (some birds might not be interactive)
impl Playable for Bird {
    fn play(&self) {
        println!("The bird is chirping happily but doesn't play.");
    }
}

// Implement the combined Animal trait
impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Bird {
    fn name(&self) -> &'static str {
        "Bird"
    }
}

// Helper functions that work with trait objects
fn feed_animal(animal: &dyn Feedable) {
    animal.feed();
}

fn play_with_animal(animal: &dyn Playable) {
    animal.play();
}

// Function that handles complete animal interaction
fn interact_with_animal(animal: &dyn Animal) {
    println!("Interacting with {}", animal.name());
    animal.feed();
    animal.play();
    println!();
}

fn main() {
    // Using trait objects to store different animal types
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Bird),
    ];

    // Demonstrate polymorphic behavior
    println!("=== Individual Behaviors ===");
    for animal in &animals {
        feed_animal(animal.as_ref());
        play_with_animal(animal.as_ref());
        println!();
    }

    // Demonstrate combined behavior
    println!("=== Complete Interactions ===");
    for animal in &animals {
        interact_with_animal(animal.as_ref());
    }

    // Demonstrate flexibility: create a collection of only feedable animals
    println!("=== Feeding Time ===");
    let feedable_animals: Vec<&dyn Feedable> = vec![&Dog, &Cat, &Bird];
    for animal in feedable_animals {
        feed_animal(animal);
    }
}
