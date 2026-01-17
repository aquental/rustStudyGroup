use super::*;

#[test]
fn test_process_tokens() {
    let tokens = vec![
        Token::Number(5),
        Token::Number(-1),
        Token::Number(10),
        Token::End,
    ];
    let result = process_tokens(tokens);
    assert_eq!(result, vec![5, 10]);
}