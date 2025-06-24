/*
Focus on the Dependency Inversion Principle from the SOLID principles.
The code (./src/dep_inv.rs) involves a printer system that directly interacts with specific printer implementations, leading to tight coupling. Your task is to refactor the code by introducing an abstraction that allows the printer system to depend not on concrete printer implementations but on an abstraction.
Make the codebase more flexible and easier to extend or modify by promoting loose coupling.

Tasks are:
- Create a trait that defines the printing behavior;
- Implement two concrete printer types: InkjetPrinter and LaserPrinter;
- Create a PrintManager struct that can be constructed with a printer and supports changing printers via set_printer and printing documents via print_document;
- Demonstrate the usage in the main function by creating both printer types and using them through the PrintManager.
- Remember, the goal is to implement the Dependency Inversion Principle by making high-level modules (like PrintManager) depend on abstractions rather than concrete implementations. This means that you need to ensure the PrintManager can work with any printer type that implements your trait. This will allow different printer implementations to be swapped out without altering the printer system's code.
*/
// Create a trait that defines the printing behavior
trait Printer {
    fn print(&self, document: &str);
}

// Implement concrete printer types
struct InkjetPrinter;

impl Printer for InkjetPrinter {
    fn print(&self, document: &str) {
        println!("Inkjet Printer: Printing {}", document);
    }
}

struct LaserPrinter;

impl Printer for LaserPrinter {
    fn print(&self, document: &str) {
        println!("Laser Printer: Printing {}", document);
    }
}

// PrintManager now depends on the abstraction (Printer trait) rather than concrete implementations
struct PrintManager {
    printer: Box<dyn Printer>,
}

impl PrintManager {
    // Constructor that accepts any printer that implements the Printer trait
    fn new(printer: Box<dyn Printer>) -> Self {
        PrintManager { printer }
    }

    // Method to change the printer at runtime
    fn set_printer(&mut self, printer: Box<dyn Printer>) {
        self.printer = printer;
    }

    // Method to print documents using the current printer
    fn print_document(&self, document: &str) {
        self.printer.print(document);
    }
}

fn main() {
    // Create concrete printer instances
    let inkjet_printer = Box::new(InkjetPrinter);
    let laser_printer = Box::new(LaserPrinter);
    
    // Demonstrate PrintManager with InkjetPrinter
    let mut manager = PrintManager::new(inkjet_printer);
    manager.print_document("Hello World!");
    
    // Change printer at runtime and print another document
    manager.set_printer(laser_printer);
    manager.print_document("SOLID Principles Rock!");
    
    // Demonstrate that we can easily switch back
    let another_inkjet = Box::new(InkjetPrinter);
    manager.set_printer(another_inkjet);
    manager.print_document("Flexible printing system!");
}
