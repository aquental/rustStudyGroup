use crate::logger::Logger;

// Define a trait Loan
pub trait Loan {
    // Define a method signature named display
    fn display(&self);
}

// Define a HomeLoan struct implementing Loan
pub struct HomeLoan;

impl Loan for HomeLoan {
    // Implement the display method
    fn display(&self) {
        Logger::instance().log("Home Loan created.");
    }
}

// Define a CarLoan struct implementing Loan
pub struct CarLoan;

impl Loan for CarLoan {
    // Implement the display method
    fn display(&self) {
        Logger::instance().log("Car Loan created.");
    }
}

// Optional: Add a test module to verify behavior
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loans() {
        let home_loan = HomeLoan;
        let car_loan = CarLoan;
        home_loan.display(); // Should log "Home Loan created."
        car_loan.display();  // Should log "Car Loan created."
    }
}
