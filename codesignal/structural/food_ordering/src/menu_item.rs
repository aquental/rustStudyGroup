// TODO: Define a trait MenuComponent
// - Create a trait MenuComponent with the methods show_details() and price().
pub trait MenuComponent {
    fn show_details(&self);
    fn price(&self) -> f32;
}

// TODO: Implement MenuItem struct implementing MenuComponent
// - Implement MenuItem struct with name and item_price attributes.
// - Ensure the constructor properly initializes these attributes.
// - Implement the methods show_details() and price() in the MenuItem struct.
pub struct MenuItem {
    name: String,
    item_price: f32,
}
impl MenuItem {
    pub fn new(name: String, item_price: f32) -> Self {
        MenuItem { name, item_price }
    }
}
impl MenuComponent for MenuItem {
    fn show_details(&self) {
        println!("Name: {}", self.name);
        println!("Price: {}", self.item_price);
    }
    fn price(&self) -> f32 {
        self.item_price
    }
}
