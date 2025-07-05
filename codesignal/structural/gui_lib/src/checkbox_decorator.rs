use crate::checkbox::Checkbox;

pub struct CheckboxWithLabel {
    checkbox: Box<dyn Checkbox>,
    label: String,
}

impl CheckboxWithLabel {
    // TODO: Implement a constructor to initialize CheckboxWithLabel with a Checkbox and label
    // - Define a constructor to take a box of Checkbox and a label string.
    pub fn new(checkbox: Box<dyn Checkbox>, label: String) -> Self {
        Self { checkbox, label }
    }
}

impl Checkbox for CheckboxWithLabel {
    // TODO: Implement the render method to add label behavior
    // - Override the render method to call the base render method and display "Label: " followed by the label.
    fn render(&self) {
        self.checkbox.render();
        println!("Label: {}", self.label);
    }
}
