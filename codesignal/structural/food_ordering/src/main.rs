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

    // TODO: Create adapter instances
    // - Instantiate UberEatsAdapter, DoorDashAdapter, GrubHubAdapter.

    // TODO: Process orders
    // - Use each adapter to process orders.

    // TODO: Create individual menu items
    // - Instantiate MenuItem for "Burger" and "Fries".

    // TODO: Create and populate a combo meal
    // - Instantiate MealCombo.
    // - Add "Burger" and "Fries" to the combo.

    // TODO: Apply decorators
    // - Decorate "Burger" with CheeseDecorator.
    // - Decorate combo meal with ChiliSauceDecorator.

    // TODO: Show details and prices
    // - Show details of the combo with chili sauce.
    // - Print total price of the combo with chili sauce.
}
