use crate::strategies::TextFormatStrategy;

// TODO: Define the TextFormatter struct
// - Add a method `set_format_strategy` that accepts a Box<dyn TextFormatStrategy>
// - Add a method `format_text` that formats text using the current strategy
// - Ensure the current strategy's `format` method is applied to the text if the strategy is set
