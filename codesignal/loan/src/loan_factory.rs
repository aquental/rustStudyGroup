use crate::loan::{CarLoan, HomeLoan, Loan};

// Define a trait LoanFactory
pub trait LoanFactory {
    // Define a method signature named create_loan
    fn create_loan(&self) -> Box<dyn Loan>;
}

// Define HomeLoanFactory struct implementing LoanFactory
pub struct HomeLoanFactory;

impl LoanFactory for HomeLoanFactory {
    // Implement the create_loan method
    fn create_loan(&self) -> Box<dyn Loan> {
        Box::new(HomeLoan)
    }
}

// Define CarLoanFactory struct implementing LoanFactory
pub struct CarLoanFactory;

impl LoanFactory for CarLoanFactory {
    // Implement the create_loan method
    fn create_loan(&self) -> Box<dyn Loan> {
        Box::new(CarLoan)
    }
}

// Optional: Add a test module to verify behavior
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_factories() {
        let home_factory = HomeLoanFactory;
        let car_factory = CarLoanFactory;
        let home_loan = home_factory.create_loan();
        let car_loan = car_factory.create_loan();
        home_loan.display(); // Should log "Home Loan created."
        car_loan.display();  // Should log "Car Loan created."
    }
}
