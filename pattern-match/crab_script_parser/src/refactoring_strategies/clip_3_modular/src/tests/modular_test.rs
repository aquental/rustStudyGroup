use super::*;

#[test]
fn test_validate_input_valid() {
    let tokens = vec![
        Token::Keyword("let"),
        Token::Identifier("x".to_string()),
        Token::Number(42),
    ];
    assert!(validate_input(&tokens));
}

#[test]
fn test_validate_input_invalid() {
    let tokens = vec![Token::Keyword("if"), Token::Number(42)];
    assert!(!validate_input(&tokens));
}

#[test]
fn test_parse_declaration_valid() {
    let tokens = vec![
        Token::Keyword("let"),
        Token::Identifier("x".to_string()),
        Token::Number(42),
    ];
    assert_eq!(parse_declaration(&tokens), Some("Declared x = 42".to_string()));
}