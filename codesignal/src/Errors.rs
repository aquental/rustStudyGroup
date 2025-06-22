use std::fmt;

// Defining custom error types with contextual information
#[derive(Debug)]
enum RepositoryError {
    AccountNotFound { account_id: String },
    InsufficientFunds { account_id: String, requested: f64, available: f64 },
}

#[derive(Debug)]
enum ServiceError {
    WithdrawalFailed { from_account_id: String, cause: RepositoryError },
    DepositFailed { to_account_id: String, cause: RepositoryError },
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::AccountNotFound { account_id } => {
                write!(f, "Account not found: {}", account_id)
            }
            RepositoryError::InsufficientFunds { account_id, requested, available } => {
                write!(
                    f,
                    "Insufficient funds in account {}: requested {}, available {}",
                    account_id, requested, available
                )
            }
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::WithdrawalFailed { from_account_id, cause } => {
                write!(
                    f,
                    "Withdrawal failed for account {}: {}",
                    from_account_id, cause
                )
            }
            ServiceError::DepositFailed { to_account_id, cause } => {
                write!(f, "Deposit failed for account {}: {}", to_account_id, cause)
            }
        }
    }
}

// Implementing error conversion from RepositoryError to ServiceError
impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> ServiceError {
        match err {
            RepositoryError::AccountNotFound { account_id } => ServiceError::WithdrawalFailed {
                from_account_id: account_id.clone(),
                cause: RepositoryError::AccountNotFound { account_id },
            },
            RepositoryError::InsufficientFunds { account_id, requested, available } => {
                ServiceError::WithdrawalFailed {
                    from_account_id: account_id.clone(),
                    cause: RepositoryError::InsufficientFunds {
                        account_id,
                        requested,
                        available,
                    },
                }
            }
        }
    }
}

struct AccountRepository;

impl AccountRepository {
    fn withdraw(&self, account_id: &str, amount: f64) -> Result<(), RepositoryError> {
        if account_id == "11111" {
            return Err(RepositoryError::AccountNotFound {
                account_id: account_id.to_string(),
            });
        }
        if amount > 100.0 {
            return Err(RepositoryError::InsufficientFunds {
                account_id: account_id.to_string(),
                requested: amount,
                available: 100.0,
            });
        }
        Ok(())
    }

    fn deposit(&self, account_id: &str, _amount: f64) -> Result<(), RepositoryError> {
        if account_id == "11111" {
            return Err(RepositoryError::AccountNotFound {
                account_id: account_id.to_string(),
            });
        }
        Ok(())
    }
}

struct AccountService {
    account_repository: AccountRepository,
}

impl AccountService {
    fn transfer_funds(
        &self,
        from_account_id: &str,
        to_account_id: &str,
        amount: f64,
    ) -> Result<(), ServiceError> {
        self.account_repository
            .withdraw(from_account_id, amount)
            .map_err(|e| ServiceError::WithdrawalFailed {
                from_account_id: from_account_id.to_string(),
                cause: e,
            })?;
        self.account_repository
            .deposit(to_account_id, amount)
            .map_err(|e| ServiceError::DepositFailed {
                to_account_id: to_account_id.to_string(),
                cause: e,
            })?;
        Ok(())
    }
}

fn main() {
    let account_service = AccountService {
        account_repository: AccountRepository,
    };

    match account_service.transfer_funds("12345", "67890", 150.00) {
        Ok(_) => println!("Transaction successful"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    // Example with non-existing account
    match account_service.transfer_funds("11111", "67890", 50.00) {
        Ok(_) => println!("Transaction successful"),
        Err(e) => println!("Transaction failed: {}", e),
    }
}
