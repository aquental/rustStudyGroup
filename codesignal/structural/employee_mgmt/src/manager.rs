use crate::employee::Employee;

pub struct Manager {
    employees: Vec<Box<dyn Employee>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            employees: Vec::new(),
        }
    }

    pub fn add(&mut self, employee: Box<dyn Employee>) {
        self.employees.push(employee);
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.employees.len() {
            self.employees.remove(index);
        }
    }
}

impl Employee for Manager {
    fn display(&self, depth: usize) {
        // Display manager information with current depth indentation
        println!("{}Manager:", "  ".repeat(depth));
        
        // Display each employee with increased depth for hierarchy
        for employee in &self.employees {
            employee.display(depth + 1);
        }
    }
}
