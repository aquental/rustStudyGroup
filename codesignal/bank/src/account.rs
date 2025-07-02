use crate::logger::Logger;

// Account hierarchy
pub trait Account {
    fn display(&self);
}

pub struct SavingsAccount;

impl Account for SavingsAccount {
    fn display(&self) {
        Logger::log("Savings Account created.".to_string());
    }
}

pub struct CurrentAccount;

impl Account for CurrentAccount {
    fn display(&self) {
        Logger::log("Current Account created.".to_string());
    }
}
