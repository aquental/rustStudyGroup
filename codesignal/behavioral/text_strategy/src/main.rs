mod formatter;
mod strategies;

use formatter::TextFormatter;
use strategies::{LowerCaseFormat, UpperCaseFormat};

fn main() {
    // TODO: Create UpperCaseFormat and LowerCaseFormat instances
    // TODO: Create a TextFormatter instance
    let upper = UpperCaseFormat::new();
    let lower = LowerCaseFormat::new();
    let mut formatter = TextFormatter::new();

    // TODO: Define a sample text "Hello, World!"
    let sample = "Hello, World!".to_string();

    // TODO: Set the format strategy of the TextFormatter to UpperCaseFormat
    // - Format the text
    // - Print the formatted text
    formatter.set_format_strategy(Box::new(upper));
    let formatted = formatter.format_text(&sample);
    println!("{}", formatted);

    // TODO: Reset the sample text to "Hello, World!"
    //let sample = "Hello, World!".to_string();

    // TODO: Set the format strategy of the TextFormatter to LowerCaseFormat
    // - Format the text
    // - Print the formatted text
    formatter.set_format_strategy(Box::new(lower));
    let formatted = formatter.format_text(&sample);
    println!("{}", formatted);
}
