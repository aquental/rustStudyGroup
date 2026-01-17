use super::*;

#[test]
fn test_parse_declaration_valid() {
    let tokens = vec![
        Token::Keyword("let"),
        Token::Identifier("x".to_string()),
        Token::Number(42),
    ];
    let result = parse_declaration(&tokens);
    assert_eq!(result, Some("Declared x = 42".to_string()));
}

#[test]
fn test_parse_declaration_invalid_empty_name() {
    let tokens = vec![
        Token::Keyword("let"),
        Token::Identifier("".to_string()),
        Token::Number(42),
    ];
    let result = parse_declaration(&tokens);
    assert_eq!(result, None);
}

#[test]
fn test_parse_declaration_invalid_zero() {
    let tokens = vec![
        Token::Keyword("let"),
        Token::Identifier("y".to_string()),
        Token::Number(0),
    ];
    let result = parse_declaration(&tokens);
    assert_eq!(result, None);
}