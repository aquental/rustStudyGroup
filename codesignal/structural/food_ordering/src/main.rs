mod food_delivery;
mod food_delivery_adapter;
mod menu_composite;
mod menu_decorator;
mod menu_item;

use food_delivery::*;
use food_delivery_adapter::*;
use menu_composite::*;
use menu_decorator::*;
use menu_item::*;

fn main() {
    // TODO: Create instances of UberEats, DoorDash, and GrubHub
    // - Instantiate UberEats, DoorDash, and GrubHub.
    let ueats = UberEats::new();
    let ddash = DoorDash::new();
    let ghub = GrubHub::new();

    // TODO: Create adapter instances
    // - Instantiate UberEatsAdapter, DoorDashAdapter, GrubHubAdapter.
    let ueats_adapter = UberEatsAdapter::new(ueats);
    let ddash_adapter = DoorDashAdapter::new(ddash);
    let ghub_adapter = GrubHubAdapter::new(ghub);

    // TODO: Process orders
    // - Use each adapter to process orders.
    ueats_adapter.process_order("Burger");
    ddash_adapter.process_order("Burger");
    ghub_adapter.process_order("Burger");

    // TODO: Create individual menu items
    // - Instantiate MenuItem for "Burger" and "Fries".
    let burger = MenuItem::new(String::from("Burger"), 10.0);
    let fries = MenuItem::new(String::from("Fries"), 5.0);

    // TODO: Create and populate a combo meal
    // - Instantiate MealCombo.
    // - Add "Burger" and "Fries" to the combo.
    let mut combo = MealCombo::new();
    combo.add(Box::new(burger));
    combo.add(Box::new(fries));

    // TODO: Apply decorators
    // - Decorate "Burger" with CheeseDecorator.
    // - Decorate combo meal with ChiliSauceDecorator.
    let burger2 = MenuItem::new(String::from("Burger"), 10.0);
    let cheese_burger = CheeseDecorator::new(Box::new(burger2));
    let chili_combo = ChiliSauceDecorator::new(Box::new(combo));

    // TODO: Show details and prices
    // - Show details of the combo with chili sauce.
    // - Print total price of the combo with chili sauce.
    chili_combo.show_details();
    println!("Total price: {}", chili_combo.price());
}
