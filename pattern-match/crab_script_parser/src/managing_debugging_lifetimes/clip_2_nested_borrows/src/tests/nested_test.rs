use super::*;

#[test]
fn test_process_nested() {
    let token = Token::Word("hello");
    let parser = Parser { current_token: &token };
    let result = process_nested(&parser);
    assert_eq!(result, "hello");
}