# Performance Analysis of the Missing Number Algorithms

## 1. Introduction

This project implements and compares two Rust algorithms for the Missing Number problem:

- Sum Algorithm
- XOR Algorithm

Both algorithms find the missing number from the sequence 1 through n, where exactly one number is missing.

## 2. Computational Complexity

### Sum Algorithm

The Sum Algorithm calculates the expected sum using `n * (n + 1) / 2`, then subtracts the sum of the supplied numbers.

- **Time complexity:** O(n), because it traverses the input vector once.
- **Auxiliary space complexity:** O(1), because it uses only a constant number of additional variables.

### XOR Algorithm

The XOR Algorithm XORs all values from 1 through n, then XORs all values in the input vector.

- **Time complexity:** O(n), because it processes the expected sequence and the input vector.
- **Auxiliary space complexity:** O(1), because it uses only a constant amount of additional working memory.

The input vector itself requires O(n) storage and is not included in the auxiliary-space figure.

## 3. Benchmark Results

Each algorithm was executed 100 times for each input size. The reported times are the total elapsed time for those 100 executions.

| Input size | Sum algorithm | XOR algorithm |
|---:|---:|---:|
| 1,000 | 26.3 µs | 151.4 µs |
| 10,000 | 246.7 µs | 1.7021 ms |
| 100,000 | 3.2224 ms | 19.3525 ms |

In these runs, the Sum implementation was faster for all three input sizes. Although both algorithms have O(n) time complexity, the Sum implementation traverses the input vector once. The XOR implementation processes the expected sequence and then traverses the input vector, so it performs more work.

These are measurements from one benchmark run and should not be treated as universal performance results. Execution time can vary with system load, processor frequency, compiler version, and other environmental factors.

## 4. Vector Memory Measurement

The benchmark measured the input vector's reserved element storage using:

`numbers.capacity() * size_of::<u64>()`

A `u64` occupies 8 bytes. The measured results were:

| Input size | Vector capacity | Reserved vector storage |
|---:|---:|---:|
| 1,000 | 1,024 elements | 8,192 bytes (8 KiB) |
| 10,000 | 16,384 elements | 131,072 bytes (128 KiB) |
| 100,000 | 131,072 elements | 1,048,576 bytes (1,024 KiB / 1 MiB) |

The capacity can exceed the number of values stored because a Rust `Vec` may reserve more space than its current length. These figures describe the vector's reserved element buffer only. They do not represent total process memory, and they do not include the vector header, stack, executable, runtime, or other allocations.

Both algorithms use the same input vector and O(1) auxiliary working space.

## 5. CPU Cache Effects

Modern CPUs use small, fast caches to keep recently or frequently accessed data close to the processor. Sequential access and reuse of the same data can benefit from cache locality.

This benchmark reuses the same input vector for repeated executions. Consequently, later executions may access data that is already in CPU cache. This can influence measured runtime. The Sum implementation reads the input vector once. The XOR implementation reads the expected sequence and then reads the input vector, resulting in an additional pass over data.

However, elapsed-time measurements alone cannot establish the number of cache hits or cache misses. Cache hit rates and cache-miss counts were **not directly measured** in this experiment. A hardware performance profiler and supported CPU performance counters would be required to measure them. Therefore, no numerical cache-hit or cache-miss claims are made.

## 6. Conclusion

Both implementations have O(n) time complexity and O(1) auxiliary space complexity, excluding the input vector. In the measured runs, the Sum implementation was faster than the XOR implementation. The vector's reserved storage increased with input size, as expected.

The results show that asymptotic complexity does not by itself determine observed runtime. The number of passes over the data, memory access patterns, compiler optimizations, CPU frequency, and caching can all affect practical performance. Direct cache-counter measurements would be needed to quantify the cache effects.

## 7. Experimental Environment

- Language: Rust
- Build profile: optimized Cargo bench profile
- Processor: 13th Gen Intel(R) Core(TM) i5-1335U
- Operating system: Windows
- Repetitions: 100 executions per algorithm per input size
