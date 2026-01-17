// CrabScript parser refactoring if-let chains
#[derive(Debug, PartialEq)]
enum Token {
    Keyword(&'static str),
    Number(i32),
}

pub fn process_input(input: Option<Result<i32, &'static str>>) -> String {
    // Objective: Replace nested if-let with match for cleaner code
    // Task: Handle Option<Result> and return a message
    // Hint: Use multiple match arms

    let mut message = String::new();
    match input {
        Some(Ok(num)) if num > 0 => message = format!("Positive: {}", num),
        Some(Ok(num)) => message = format!("Non-positive: {}", num),
        Some(Err(err)) => message = format!("Error: {}", err),
        None => message = "No data".to_string(),
    }

    message
}
