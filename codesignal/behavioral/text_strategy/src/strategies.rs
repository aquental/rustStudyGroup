pub trait TextFormatStrategy {
    fn format(&self, text: &str) -> String;
}

// TODO: Define the UpperCaseFormat struct implementing TextFormatStrategy
// - The `format` method should convert the given text to uppercase
pub struct UpperCaseFormat;
impl UpperCaseFormat {
    pub fn new() -> Self {
        Self
    }
}
impl TextFormatStrategy for UpperCaseFormat {
    fn format(&self, text: &str) -> String {
        text.to_uppercase()
    }
}

// TODO: Define the LowerCaseFormat struct implementing TextFormatStrategy
// - The `format` method should convert the given text to lowercase
pub struct LowerCaseFormat;
impl LowerCaseFormat {
    pub fn new() -> Self {
        Self
    }
}
impl TextFormatStrategy for LowerCaseFormat {
    fn format(&self, text: &str) -> String {
        text.to_lowercase()
    }
}
