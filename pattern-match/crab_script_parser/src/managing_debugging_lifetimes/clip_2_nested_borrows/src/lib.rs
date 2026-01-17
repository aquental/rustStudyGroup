// CrabScript parser with nested lifetime management
#[derive(Debug)]
enum Token<'a> {
    Word(&'a str),
    Number(i32),
}

#[derive(Debug)]
struct Parser<'a> {
    current_token: &'a Token<'a>,
}

pub fn process_nested<'a>(parser: &'a Parser<'a>) -> &'a str {
    // Objective: Manage nested borrows with lifetime annotations
    // Task: Extract word from nested token, ensuring sub-references outlive
    // Hint: Use lifetime bounds

    match parser.current_token {
        Token::Word(word) => word,
        _ => "",
    }

}