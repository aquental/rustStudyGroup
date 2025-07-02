use crate::account::{Account, CurrentAccount, SavingsAccount};

// Abstract Factory Pattern for creating accounts
pub trait AccountFactory {
    fn create_account(&self) -> Box<dyn Account>;
}

pub struct SavingsAccountFactory;

impl AccountFactory for SavingsAccountFactory {
    fn create_account(&self) -> Box<dyn Account> {
        Box::new(SavingsAccount)
    }
}

pub struct CurrentAccountFactory;

impl AccountFactory for CurrentAccountFactory {
    fn create_account(&self) -> Box<dyn Account> {
        Box::new(CurrentAccount)
    }
}
