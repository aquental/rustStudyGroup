mod account;
mod account_factory;
mod logger;

use account_factory::{AccountFactory, CurrentAccountFactory, SavingsAccountFactory};

fn main() {
    // Create a savings account using the SavingsAccountFactory
    let savings_factory = SavingsAccountFactory;
    let savings_account = savings_factory.create_account();

    // Call the display method from the savings account
    savings_account.display();

    // Create a current account using the CurrentAccountFactory
    let current_factory = CurrentAccountFactory;
    let current_account = current_factory.create_account();

    // Call the display method from the current account
    current_account.display();
}
