use crate::strategies::TextFormatStrategy;

// TODO: Define the TextFormatter struct
// - Add a method `set_format_strategy` that accepts a Box<dyn TextFormatStrategy>
// - Add a method `format_text` that formats text using the current strategy
// - Ensure the current strategy's `format` method is applied to the text if the strategy is set
pub struct TextFormatter {
    format_strategy: Option<Box<dyn TextFormatStrategy>>,
}
impl TextFormatter {
    pub fn new() -> Self {
        Self {
            format_strategy: None,
        }
    }
    pub fn set_format_strategy(&mut self, strategy: Box<dyn TextFormatStrategy>) {
        self.format_strategy = Some(strategy);
    }
    pub fn format_text(&self, text: &str) -> String {
        match &self.format_strategy {
            Some(strategy) => strategy.format(text),
            None => text.to_string(),
        }
    }
}
