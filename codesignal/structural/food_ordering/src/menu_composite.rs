use crate::menu_item::MenuComponent;

// TODO: Implement MealCombo struct implementing MenuComponent
// - Implement a private Vec<Box<dyn MenuComponent>> items to store menu components.
// - Implement the add(item: Box<dyn MenuComponent>) method to add items to the combo.
// - Implement the remove() method to remove the last item from the combo.
// - Implement the show_details() method to display details of all items in the combo.
// - Implement the price() method to calculate the total price of the combo.
pub struct MealCombo {
    items: Vec<Box<dyn MenuComponent>>,
}

impl MealCombo {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn add(&mut self, item: Box<dyn MenuComponent>) {
        self.items.push(item);
    }
    pub fn remove(&mut self) {
        self.items.pop();
    }
}
impl MenuComponent for MealCombo {
    fn show_details(&self) {
        for item in &self.items {
            item.show_details();
        }
    }
    fn price(&self) -> f32 {
        self.items.iter().fold(0.0, |acc, item| acc + item.price())
    }
}
