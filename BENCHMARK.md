# Missing Number – Benchmark and Interpretation

## 1. Objective

The objective is to find the missing number from the sequence 1, 2, ..., n using Rust.

This project compares two algorithms:
- Sum Algorithm
- XOR Algorithm

## 2. Algorithms

### Sum Algorithm

The missing number is calculated using the mathematical formula:

Missing Number = n × (n + 1) / 2 − Sum of Given Numbers

### XOR Algorithm

The XOR algorithm applies the XOR operation to all expected numbers and all given numbers. Matching numbers cancel each other, leaving the missing number.

## 3. Time Complexity

Both algorithms have O(n) time complexity.

- Sum Algorithm: Iterates through the given numbers once.
- XOR Algorithm: Iterates through the expected numbers and the given numbers separately.

## 4. Space Complexity

Both algorithms require O(1) auxiliary space for the missing-number calculation.

The input vector requires O(n) space because it stores the given numbers.

## 5. Benchmark Setup

The benchmark was performed using the optimized Rust Cargo bench profile.

Each algorithm was executed 100 times for each input size.

Input sizes:
- 1,000
- 10,000
- 100,000

Processor: 13th Gen Intel Core i5-1335U  
Operating System: Windows

## 6. Latest Benchmark Results

| Input Size | Sum Algorithm (100 runs) | XOR Algorithm (100 runs) |
|---:|---:|---:|
| 1,000 | 26.3 µs | 151.4 µs |
| 10,000 | 246.7 µs | 1.7021 ms |
| 100,000 | 3.2224 ms | 19.3525 ms |

These are total elapsed times for 100 executions of each algorithm.

## 7. Interpretation

The benchmark results show that the Sum Algorithm was faster than the XOR Algorithm for all three tested input sizes.

Both algorithms have O(n) time complexity. However, the XOR implementation performs two separate iterations, while the Sum implementation iterates through the input numbers once.

Execution time increased as the input size increased.

The benchmark repeatedly uses the same input vector. Therefore, CPU caching may influence the measurements. However, cache hits and misses were not measured directly.

## 8. Memory Usage

The input vector stores values as `u64`, with each value requiring 8 bytes.

Memory was estimated using the vector's capacity:

`numbers.capacity() * size_of::<u64>()`

| Input Size | Vector Capacity | Reserved Vector Memory |
|---:|---:|---:|
| 1,000 | 1,024 elements | 8,192 bytes (8 KiB) |
| 10,000 | 16,384 elements | 131,072 bytes (128 KiB) |
| 100,000 | 131,072 elements | 1,048,576 bytes (1 MiB) |

The vector capacity can be larger than its length because Rust may reserve additional storage as a vector grows.

These figures represent the reserved element buffer of the input vector, not the total memory consumption of the application.

Both algorithms use O(1) auxiliary space, excluding the O(n) input vector.

## 9. CPU Caching Effects

Modern processors use CPU caches to access frequently used data faster than main memory.

The benchmark repeatedly uses the same input vector. This may allow data to remain in CPU cache between executions and influence the measured execution times.

Both algorithms access data sequentially. However, the Sum Algorithm reads the input vector once, while the XOR Algorithm processes the expected sequence and then reads the input vector.

Cache hits and cache misses were not directly measured in this experiment. Hardware performance profiling would be required to obtain actual cache-counter measurements.

Therefore, no numerical cache hit rate or cache miss count is claimed.

## 10. Conclusion

Both algorithms successfully solve the Missing Number problem.

Both have O(n) time complexity and O(1) auxiliary space complexity, excluding the input vector.

Based on the measured results, the Sum Algorithm performed faster than the XOR Algorithm for the tested input sizes.

The experiment demonstrates that algorithms with the same asymptotic complexity can have different practical execution times because of differences in the number of operations, memory access patterns, compiler optimizations, and CPU caching behavior.

Further hardware-level profiling would be required to quantify the exact cache effects.
