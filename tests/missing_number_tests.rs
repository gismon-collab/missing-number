
use missing_number::{missing_number, missing_number_xor};

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

#[test]
fn test_xor_missing_middle() {
    assert_eq!(missing_number_xor(5, &[1, 2, 3, 5]), 4);
}

#[test]
fn test_xor_missing_first() {
    assert_eq!(missing_number_xor(5, &[2, 3, 4, 5]), 1);
}

#[test]
fn test_xor_missing_last() {
    assert_eq!(missing_number_xor(5, &[1, 2, 3, 4]), 5);
}
