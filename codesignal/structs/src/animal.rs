// Define a trait 'Animal' with a method 'sound' that returns a String
trait Animal {
    fn sound(&self) -> String;
}

// Create a struct 'Dog'
struct Dog;

// Implement the 'Animal' trait for 'Dog'
// - The 'sound' method should return "Bark"
impl Animal for Dog {
    fn sound(&self) -> String {
        "Bark".to_string()
    }
}

// Create a struct 'Cat'
struct Cat;

// Implement the 'Animal' trait for 'Cat'
// - The 'sound' method should return "Meow"
impl Animal for Cat {
    fn sound(&self) -> String {
        "Meow".to_string()
    }
}

// Write a function 'print_sound' that takes an argument of type &dyn Animal
// - Print the sound returned by the 'sound' method
fn print_sound(animal: &dyn Animal) {
    println!("{}", animal.sound());
}

fn main() {
    // Create instances of 'Dog' and 'Cat'
    let dog = Dog;
    let cat = Cat;

    // Use 'print_sound' to print the sounds of Dog and Cat
    print_sound(&dog);
    print_sound(&cat);
}
