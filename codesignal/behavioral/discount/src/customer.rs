// TODO: Define the Customer struct
// - Constructor parameter: name
// - Method get_name() -> String: returns the customer's name
// - Method update(status: &str): prints "{customer_name} received update: {status}"
pub struct Customer {
    pub name: String,
}

impl Customer {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn update(&self, status: &str) {
        println!("{} received update: {}", self.name, status);
    }
}
