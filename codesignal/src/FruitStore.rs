// Define a common Fruit trait
trait Fruit {
    fn description(&self) -> String;
}

struct Apple;

// Implement the Fruit trait for Apple
impl Fruit for Apple {
    fn description(&self) -> String {
        String::from("This is an apple.")
    }
}

struct Banana;

// Implement the Fruit trait for Banana
impl Fruit for Banana {
    fn description(&self) -> String {
        String::from("This is a banana.")
    }
}
struct Orange;

impl Fruit for Orange {
    fn description(&self) -> String {
        String::from("This is an orange.")
    }
}

// Factory component that handles fruit creation
struct FruitFactory;

impl FruitFactory {
    fn create_fruit(&self, fruit_type: &str) -> Option<Box<dyn Fruit>> {
        match fruit_type.to_lowercase().as_str() {
            "apple" => Some(Box::new(Apple)),
            "banana" => Some(Box::new(Banana)),
            "orange" => Some(Box::new(Orange)),
            _ => None,
        }
    }
}

struct FruitStore {
    factory: FruitFactory,
}

impl FruitStore {
    fn new() -> Self {
        FruitStore {
            factory: FruitFactory,
        }
    }

    // Delegate fruit creation to the factory
    fn order_fruit(&self, fruit_type: &str) -> Option<Box<dyn Fruit>> {
        self.factory.create_fruit(fruit_type)
    }
}

fn main() {
    let fruit_store = FruitStore::new();

    // Order and describe an Apple via the FruitStore
    if let Some(apple) = fruit_store.order_fruit("apple") {
        println!("{}", apple.description());
    }

    // Order and describe a Banana via the FruitStore
    if let Some(banana) = fruit_store.order_fruit("banana") {
        println!("{}", banana.description());
    }

    // Demonstrate error handling for unsupported fruit types
    if let Some(orange) = fruit_store.order_fruit("orange") {
        println!("{}", orange.description());
    } else {
        println!("Sorry, orange is not available in our store.");
    }
}
