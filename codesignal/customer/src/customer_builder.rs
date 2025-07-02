use crate::customer::{Customer, PremiumCustomer, RegularCustomer};

pub trait CustomerBuilder {
    fn build_name(&mut self, name: String) -> &mut Self;
    fn build_email(&mut self, email: String) -> &mut Self;
    fn build_extra_details(&mut self, details: &str) -> &mut Self;
    fn build(&self) -> Box<dyn Customer>;
}

pub struct PremiumCustomerBuilder {
    name: Option<String>,
    email: Option<String>,
    tier: Option<String>,
}

impl PremiumCustomerBuilder {
    pub fn new() -> Self {
        PremiumCustomerBuilder {
            name: None,
            email: None,
            tier: None,
        }
    }
}

impl CustomerBuilder for PremiumCustomerBuilder {
    fn build_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    fn build_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    fn build_extra_details(&mut self, details: &str) -> &mut Self {
        self.tier = Some(details.to_string());
        self
    }

    fn build(&self) -> Box<dyn Customer> {
        Box::new(PremiumCustomer {
            name: self.name.clone().unwrap_or_default(),
            email: self.email.clone().unwrap_or_default(),
            tier: self.tier.clone().unwrap_or_default(),
        })
    }
}

pub struct RegularCustomerBuilder {
    name: Option<String>,
    email: Option<String>,
    loyalty_status: Option<bool>,
}

impl RegularCustomerBuilder {
    pub fn new() -> Self {
        RegularCustomerBuilder {
            name: None,
            email: None,
            loyalty_status: None,
        }
    }
}

impl CustomerBuilder for RegularCustomerBuilder {
    fn build_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    fn build_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    fn build_extra_details(&mut self, details: &str) -> &mut Self {
        self.loyalty_status = Some(details.to_lowercase() == "true");
        self
    }

    fn build(&self) -> Box<dyn Customer> {
        Box::new(RegularCustomer {
            name: self.name.clone().unwrap_or_default(),
            email: self.email.clone().unwrap_or_default(),
            loyalty_status: self.loyalty_status.unwrap_or(false),
        })
    }
}
