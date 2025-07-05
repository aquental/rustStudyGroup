mod checkbox;
mod checkbox_adapter;
mod checkbox_decorator;

use checkbox::{Checkbox, MacCheckbox};
use checkbox_adapter::MacToWinCheckboxAdapter;
use checkbox_decorator::CheckboxWithLabel;

fn main() {
    // TODO: Create instances of MacCheckbox and MacToWinCheckboxAdapter
    let mac_checkbox = MacCheckbox {};
    let mac_to_win_checkbox_adapter = MacToWinCheckboxAdapter::new(mac_checkbox);

    // TODO: Create an instance of CheckboxWithLabel using MacToWinCheckboxAdapter
    let checkbox_with_label = CheckboxWithLabel::new(
        Box::new(mac_to_win_checkbox_adapter),
        "Checkbox".to_string(),
    );

    // TODO: Render the CheckboxWithLabel
    checkbox_with_label.render();
}
