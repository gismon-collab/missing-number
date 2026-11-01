\# Missing Number – Benchmark and Interpretation



\## 1. Objective

The main objective is to find the missing number from the sequence 1, 2, ..., n using Rust.



\## 2. Algorithm

The missing number is calculated using the mathematical formula:



Missing Number = n × (n + 1) / 2 − Sum of Given Numbers



\## 3. Time Complexity

\*\*O(n)\*\*



The algorithm iterates through the given numbers once to calculate their sum.



\## 4. Space Complexity

\*\*O(n)\*\* for storing the input numbers in the vector used by this implementation.



The missing-number calculation itself requires O(1) auxiliary space.



\## 5. Benchmark Results

The benchmark was performed using optimized Rust benchmark mode. Each input size was tested 100 times.

| Input Size | Time for 100 Runs |
|---|---:|
| 2,000 | 33.9 µs |
| 50,000 | 1.0513 ms |
| 200,000 | 4.5072 ms |

## 6. Interpretation

The benchmark results show that execution time increases as the input size increases.

- 2,000 numbers: 33.9 microseconds for 100 runs.
- 50,000 numbers: 1.0513 milliseconds for 100 runs.
- 200,000 numbers: 4.5072 milliseconds for 100 runs.

The algorithm efficiently processes the input using a single iteration to calculate the sum.

## 7. Conclusion

The Missing Number problem was successfully implemented using Rust and the mathematical sum formula.

All 3 integration tests passed successfully. The benchmark was also completed successfully. The solution is suitable for the given constraint of n ≤ 200,000.



