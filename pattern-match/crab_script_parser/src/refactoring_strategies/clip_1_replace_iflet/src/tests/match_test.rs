use super::*;

#[test]
fn test_process_input_positive() {
    let input = Some(Ok(42));
    assert_eq!(process_input(input), "Positive: 42");
}

#[test]
fn test_process_input_non_positive() {
    let input = Some(Ok(0));
    assert_eq!(process_input(input), "Non-positive: 0");
}

#[test]
fn test_process_input_error() {
    let input = Some(Err("parse error"));
    assert_eq!(process_input(input), "Error: parse error");
}

#[test]
fn test_process_input_none() {
    let input = None;
    assert_eq!(process_input(input), "No data");
}