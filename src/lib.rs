pub fn missing_number(n: u64, numbers: &[u64]) -> u64 {
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: u64 = numbers.iter().sum();

    expected_sum - actual_sum
}

pub fn missing_number_xor(n: u64, numbers: &[u64]) -> u64 {
    let mut result = 0;

    for i in 1..=n {
        result ^= i;
    }

    for &number in numbers {
        result ^= number;
    }

    result
}