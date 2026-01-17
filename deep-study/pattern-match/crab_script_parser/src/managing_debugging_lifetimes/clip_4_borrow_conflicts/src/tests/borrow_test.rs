use super::*;

#[test]
fn test_modify_data() {
    let mut data = vec![1, 2, 3];
    modify_data(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4]);
}