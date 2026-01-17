// CrabScript parser for handling variable-length token sequences
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Keyword(&'static str),
    Number(i32),
}

pub fn parse_tokens(tokens: &[Token]) -> Option<(Token, Vec<Token>)> {
    // Objective: Use advanced pattern matching on slices to split tokens
    // Task: Implement pattern matching to extract the first token and the rest
    // Hint: Use [first, ..] or similar slice patterns

    match tokens {
        [first, rest @ ..] => Some((first.clone(), rest.to_vec())), 
        _ => None,
    }
}

pub fn parse_iterator(tokens: Vec<Token>) -> Vec<i32> {
    // Objective: Use pattern matching with iterators to extract numbers
    // Task: Filter and collect numbers from a token iterator
    // Hint: Use while-let or for with destructuring

    let mut numbers = Vec::new();
    let mut iter = tokens.into_iter();
    while let Some (Token::Number(n)) = iter.next() {
        numbers.push(n);
    }

    numbers
}
