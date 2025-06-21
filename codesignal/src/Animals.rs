/**
Transformed code using dynamic dispatch through traits instead of enums and pattern matching.
This approach uses trait objects to enable polymorphic behavior without match statements.
Each animal type implements the Animal trait, and we use trait objects (Box<dyn Animal>)
to store different animal types in a homogeneous collection.
*/

// Define a trait that all animals must implement
trait Animal {
    fn feed(&self);
}

// Concrete animal types
struct Dog;
struct Cat;
struct Bird;

// Implement the Animal trait for each type
impl Animal for Dog {
    fn feed(&self) {
        println!("Feeding the dog with dog food.");
    }
}

impl Animal for Cat {
    fn feed(&self) {
        println!("Feeding the cat with cat food.");
    }
}

impl Animal for Bird {
    fn feed(&self) {
        println!("Feeding the bird with bird seeds.");
    }
}

// Function that works with any type implementing Animal trait
fn feed_animal(animal: &dyn Animal) {
    animal.feed();
}

fn main() {
    // Using trait objects to store different animal types
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Bird),
    ];
    
    for animal in animals.iter() {
        feed_animal(animal.as_ref());
    }
}
