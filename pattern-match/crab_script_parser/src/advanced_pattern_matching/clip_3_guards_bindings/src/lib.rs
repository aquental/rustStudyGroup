// CrabScript parser with guards and bindings for conditional parsing
#[derive(Debug, PartialEq)]
enum Token {
    Keyword(&'static str),
    Number(i32),
    Identifier(String),
}

pub fn parse_declaration(tokens: &[Token]) -> Option<String> {
    // Objective: Use guards and bindings to validate variable declarations
    // Task: Match a "let" keyword followed by an identifier and number with conditions
    // Hint: Use if guards and or patterns

    match tokens {
        [Token::Keyword("let"), Token::Identifier(ref name), Token::Number(val)]
        if !name.is_empty() && *val >= 0 => {
            Some(format!("Declared {} = {}", name, val))
        }
        _ => None,
    }
 

}

