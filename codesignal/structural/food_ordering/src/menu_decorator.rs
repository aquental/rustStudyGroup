// TODO: Remember to add necessary use (import) statements for the MenuComponent trait
use crate::menu_item::MenuComponent;
// TODO: Implement a base struct MenuItemDecorator implementing MenuComponent
// - Create a struct MenuItemDecorator that implements MenuComponent and has a protected Box<dyn MenuComponent> item.
// - Ensure the constructor properly initializes the MenuComponent item.
pub struct MenuItemDecorator {
    item: Box<dyn MenuComponent>,
}
impl MenuItemDecorator {
    pub fn new(item: Box<dyn MenuComponent>) -> Self {
        Self { item }
    }
}

// TODO: Implement ChiliSauceDecorator struct implementing MenuComponent
// - Implement the ChiliSauceDecorator struct that adds chili sauce to a menu item.
// - Ensure the constructor properly initializes the MenuComponent item.
// - Override show_details() to display chili sauce information and adjust the price accordingly.
pub struct ChiliSauceDecorator {
    item: Box<dyn MenuComponent>,
}
impl ChiliSauceDecorator {
    pub fn new(item: Box<dyn MenuComponent>) -> Self {
        Self { item }
    }
}
impl MenuComponent for ChiliSauceDecorator {
    fn show_details(&self) {
        self.item.show_details();
        println!("Chili Sauce");
    }
    fn price(&self) -> f32 {
        self.item.price() + 0.5
    }
}

// TODO: Implement CheeseDecorator struct implementing MenuComponent
// - Implement the CheeseDecorator struct that adds cheese to a menu item.
// - Ensure the constructor properly initializes the MenuComponent item.
// - Override show_details() to display cheese information and adjust the price accordingly.
pub struct CheeseDecorator {
    item: Box<dyn MenuComponent>,
}
impl CheeseDecorator {
    pub fn new(item: Box<dyn MenuComponent>) -> Self {
        Self { item }
    }
}
impl MenuComponent for CheeseDecorator {
    fn show_details(&self) {
        self.item.show_details();
        println!("Cheese");
    }
    fn price(&self) -> f32 {
        self.item.price() + 1.0
    }
}
