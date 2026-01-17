use super::*;

#[test]
fn test_parse_tokens() {
    let tokens = vec![
        Token::Keyword("let"),
        Token::Number(42),
        Token::Keyword("in"),
    ];
    let result = parse_tokens(&tokens);
    assert_eq!(result, Some((Token::Keyword("let"), vec![Token::Number(42), Token::Keyword("in")])));
}

#[test]
fn test_parse_iterator() {
    let tokens = vec![
        Token::Number(10),
        Token::Keyword("if"),
        Token::Number(20),
    ];
    let result = parse_iterator(tokens);
    assert_eq!(result, vec![10, 20]);
}