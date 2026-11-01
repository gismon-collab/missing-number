use missing_number::missing_number;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let sizes = [2_000, 50_000, 200_000];

    for n in sizes {
        let missing = n / 2;

        let numbers: Vec<u64> = (1..=n).filter(|&x| x != missing).collect();

        let start = Instant::now();

        for _ in 0..100 {
            black_box(missing_number(black_box(n), black_box(&numbers)));
        }

        println!(
            "Input size: {}, Time for 100 runs: {:?}",
            n,
            start.elapsed()
        );
    }
}
