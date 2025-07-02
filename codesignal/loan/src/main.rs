mod logger;
mod loan;
mod loan_factory;

use loan_factory::{CarLoanFactory, HomeLoanFactory, LoanFactory};

fn main() {
    // Create a home loan using the HomeLoanFactory and display its details
    let home_loan_factory = HomeLoanFactory;
    let home_loan = home_loan_factory.create_loan();
    home_loan.display();

    // Create a car loan using the CarLoanFactory and display its details
    let car_loan_factory = CarLoanFactory;
    let car_loan = car_loan_factory.create_loan();
    car_loan.display();
}
