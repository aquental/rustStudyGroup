#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub balance: u32,
}
impl Account {
    pub fn new(address: String) -> Account {
        Account {
            address,
            balance: 0,
        }
    }
}

fn main() {
    let acc = Account::new("home".to_string());
    println!("{:?}", acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_new() {
        let address = "test_address".to_string();
        let account = Account::new(address.clone());
        
        assert_eq!(account.address, address);
        assert_eq!(account.balance, 0);
    }

    #[test]
    fn test_account_new_with_empty_address() {
        let address = String::new();
        let account = Account::new(address.clone());
        
        assert_eq!(account.address, address);
        assert_eq!(account.balance, 0);
    }

    #[test]
    fn test_account_new_with_long_address() {
        let address = "a_very_long_address_that_might_be_used_in_real_scenarios".to_string();
        let account = Account::new(address.clone());
        
        assert_eq!(account.address, address);
        assert_eq!(account.balance, 0);
    }

    #[test]
    fn test_account_fields_are_public() {
        let address = "public_test".to_string();
        let mut account = Account::new(address.clone());
        
        // Test that we can access and modify public fields
        assert_eq!(account.address, address);
        assert_eq!(account.balance, 0);
        
        // Test that we can modify the balance since it's public
        account.balance = 100;
        assert_eq!(account.balance, 100);
        
        // Test that we can modify the address since it's public
        account.address = "modified_address".to_string();
        assert_eq!(account.address, "modified_address");
    }

    #[test]
    fn test_account_debug_trait() {
        let account = Account::new("debug_test".to_string());
        let debug_string = format!("{:?}", account);
        
        // Check that the debug string contains the expected fields
        assert!(debug_string.contains("Account"));
        assert!(debug_string.contains("address"));
        assert!(debug_string.contains("balance"));
        assert!(debug_string.contains("debug_test"));
        assert!(debug_string.contains("0"));
    }

    #[test]
    fn test_account_with_special_characters() {
        let address = "test@example.com".to_string();
        let account = Account::new(address.clone());
        
        assert_eq!(account.address, address);
        assert_eq!(account.balance, 0);
    }

    #[test]
    fn test_account_with_unicode_characters() {
        let address = "用户地址".to_string();
        let account = Account::new(address.clone());
        
        assert_eq!(account.address, address);
        assert_eq!(account.balance, 0);
    }

    #[test]
    fn test_multiple_accounts_independence() {
        let account1 = Account::new("account1".to_string());
        let account2 = Account::new("account2".to_string());
        
        assert_eq!(account1.address, "account1");
        assert_eq!(account2.address, "account2");
        assert_eq!(account1.balance, 0);
        assert_eq!(account2.balance, 0);
        
        // Ensure they are independent
        assert_ne!(account1.address, account2.address);
    }
}
