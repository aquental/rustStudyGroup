// CrabScript parser with modular refactoring
#[derive(Debug, PartialEq)]
enum Token {
    Keyword(&'static str),
    Number(i32),
    Identifier(&'static str)
}

fn validate_input(tokens: &[Token]) -> bool {
    match tokens {
        [Token::Keyword("let"), ..] => tokens.iter().any(|t| matches!(t, Token::Number(_))),
        _ => false,
    }
}

pub fn parse_declaration(tokens: &[Token]) -> Option<String> {
    // Objective: Modularize parsing with a validation step
    // Task: Use validate_input and process valid tokens
    // Hint: Break into smaller functions

    // Starting code
    if validate_input(tokens) {
        match tokens {
            [Token::Keyword("let"), Token::Identifier(ref name), Token::Number(val)]
              => Some(format!("Declared {} = {}", name, val)),
            _ => None,
        }
    } else {
        None
    }



}