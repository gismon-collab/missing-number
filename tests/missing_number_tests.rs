use missing_number::missing_number;

#[test]
fn test_missing_middle() {
    assert_eq!(missing_number(5, &[1, 2, 3, 5]), 4);
}

#[test]
fn test_missing_first() {
    assert_eq!(missing_number(5, &[2, 3, 4, 5]), 1);
}

#[test]
fn test_missing_last() {
    assert_eq!(missing_number(5, &[1, 2, 3, 4]), 5);
}
