// RustyScript parser for destructuring nested enums
#[derive(Debug, PartialEq)]
enum ParserNode {
    Number(i32),
    String(String),
    Object(Vec<(String, ParserNode)>),
}

pub fn parse_nested_object(input: ParserNode) -> Option<(String, i32)> {
    // Objective: Use advanced pattern matching to destructure nested enums
    // Task: Extract a key-value pair where the value is a Number from an Object
    // Hint: Use nested match patterns and ref bindings to avoid ownership issues

    // Starting code
    match input {
        ParserNode::Object(fields) => fields
            .into_iter()
            .find_map(|(key, value)| match value {
                ParserNode::Number(num) => Some((key, num)),
                _ => None,
            }),
        _ => None,
    }

}