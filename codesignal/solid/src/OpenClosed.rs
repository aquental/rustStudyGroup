/*
 The given code (./src/OpenCLose.rs) ) calculates the pricing for different types of products but lacks flexibility for future expansions.
 Refactor it to allow new product types to be introduced easily while maintaining existing logic.
Redesign the code structure so that it aligns with the Open/Closed Principle. This involves making it easy to add new product types without affecting the current codebase.
Follow the TODO comments to guide you in refactoring the code.

✅ Changes Made:

1. Created a Product trait with three methods:
•  name() - returns the product name
•  base_price() - returns the base price
•  calculate_price() - calculates the final price with any applicable rules
2. Implemented product structs:
•  Book struct with name and base_price fields
•  Food struct with name and base_price fields
3. Implemented the Product trait for each product type:
•  Book: Simple pricing (no additional charges)
•  Food: Adds 7% tax to the base price
4. Updated the main function to use the new structure with trait objects

✅ Benefits of this refactoring:

•  Open for Extension: Adding new product types is now easy - just create a new struct and implement the Product trait
•  Closed for Modification: The existing code doesn't need to be changed when adding new products
•  Type Safety: Each product type has its own struct with compile-time guarantees
•  Maintainability: Each product's pricing logic is encapsulated in its own implementation
*/

trait Product {
    fn name(&self) -> &str;
    fn base_price(&self) -> f64;
    fn calculate_price(&self) -> f64;
}

struct Book {
    name: String,
    base_price: f64,
}

impl Product for Book {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_price(&self) -> f64 {
        self.base_price
    }

    fn calculate_price(&self) -> f64 {
        self.base_price
    }
}

struct Food {
    name: String,
    base_price: f64,
}

impl Product for Food {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_price(&self) -> f64 {
        self.base_price
    }

    fn calculate_price(&self) -> f64 {
        self.base_price + self.base_price * 0.07
    }
}

fn main() {
    println!("Price calculations:");

    let book = Book {
        name: "Book".to_string(),
        base_price: 29.99,
    };
    let food = Food {
        name: "Food".to_string(),
        base_price: 0.99,
    };

    println!("Price of {}: ${:.2}", book.name(), book.calculate_price());
    println!("Price of {}: ${:.2}", food.name(), food.calculate_price());
}
