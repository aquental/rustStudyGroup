use super::*;

#[test]
fn test_get_slice() {
    let input = "hello";
    let result = get_slice(input);
    assert_eq!(result, "ello");
}