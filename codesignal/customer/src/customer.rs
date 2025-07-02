use crate::logger::Logger;

// Define the Customer trait with required methods
pub trait Customer {
    fn name(&self) -> &str;
    fn email(&self) -> &str;
    fn show_info(&self, logger: &Logger);
}

// Define PremiumCustomer struct implementing Customer trait
pub struct PremiumCustomer {
    pub name: String,
    pub email: String,
    pub tier: String,
}

impl Customer for PremiumCustomer {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn show_info(&self, logger: &Logger) {
        Logger::log(&format!(
            "Premium Customer: Name: {}, Email: {}, Tier: {}",
            self.name, self.email, self.tier
        ));
    }
}

// Define RegularCustomer struct implementing Customer trait
pub struct RegularCustomer {
    pub name: String,
    pub email: String,
    pub loyalty_status: bool,
}

impl Customer for RegularCustomer {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn show_info(&self, logger: &Logger) {
        Logger::log(&format!(
            "Regular Customer: Name: {}, Email: {}, Loyalty Status: {}",
            self.name, self.email, self.loyalty_status
        ));
    }
}
