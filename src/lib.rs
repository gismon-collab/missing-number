pub fn missing_number(n: u64, numbers: &[u64]) -> u64 {
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: u64 = numbers.iter().sum();

    expected_sum - actual_sum
}
