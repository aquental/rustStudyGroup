mod employee;
mod developer;
mod manager;

use developer::Developer;
use manager::Manager;
use employee::Employee;

fn main() {
    let dev1 = Developer::new("John Doe", "Senior Developer");
    let dev2 = Developer::new("Jane Smith", "Junior Developer");
    let dev3 = Developer::new("Bob Wilson", "Frontend Developer");
    let dev4 = Developer::new("Alice Brown", "Backend Developer");

    // Create a sub-manager for frontend team
    let mut frontend_manager = Manager::new();
    frontend_manager.add(Box::new(dev3));

    // Create a sub-manager for backend team
    let mut backend_manager = Manager::new();
    backend_manager.add(Box::new(dev4));

    // Create a main manager and add developers and sub-managers
    let mut main_manager = Manager::new();
    main_manager.add(Box::new(dev1));
    main_manager.add(Box::new(dev2));
    main_manager.add(Box::new(frontend_manager));
    main_manager.add(Box::new(backend_manager));

    // Call the display method for the main manager to show employee details
    println!("Company Organization Structure:");
    main_manager.display(0);
}
