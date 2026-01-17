// CrabScript parser with loop refactoring
#[derive(Debug, PartialEq)]
enum Token {
    Keyword(&'static str),
    Number(i32),
    End,
}

pub fn process_tokens(tokens: Vec<Token>) -> Vec<i32> {
    // Objective: Refactor loop control with pattern matching
    // Task: Filter positive numbers and break on End token
    // Hint: Use match with break and continue

    let mut numbers = Vec::new();
    let mut iter = tokens.into_iter();
    while let Some(token) = iter.next() {
        match token {
            Token::End => break,
            Token::Number(num) if num > 0 => numbers.push(num),
            _ => continue,
        }
    }


    numbers
}
