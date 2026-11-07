
use missing_number::{missing_number, missing_number_xor};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let sizes = [1_000, 10_000, 100_000];

    for n in sizes {
        let missing = n / 2;

        let numbers: Vec<u64> =
            (1..=n).filter(|&x| x != missing).collect();
            // Measure memory used by the input vector
let memory_bytes = numbers.capacity()
    * std::mem::size_of::<u64>();

println!("Vector capacity: {} elements", numbers.capacity());
println!("Memory usage: {} bytes", memory_bytes);
println!("Memory usage: {:.2} KB",
    memory_bytes as f64 / 1024.0);

        // Benchmark Sum Algorithm
        let start = Instant::now();

        for _ in 0..100 {
            black_box(
                missing_number(black_box(n), black_box(&numbers))
            );
        }

        let sum_time = start.elapsed();

        // Benchmark XOR Algorithm
        let start = Instant::now();

        for _ in 0..100 {
            black_box(
                missing_number_xor(black_box(n), black_box(&numbers))
            );
        }

        let xor_time = start.elapsed();

        println!("Input size: {}", n);
        println!("Sum Algorithm: {:?}", sum_time);
        println!("XOR Algorithm: {:?}", xor_time);
        println!("--------------------------");
    }
}
