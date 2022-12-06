# Advent of Code 2022

### Run tests and benchmark 

```
# Run tests 
cd d01/a
cargo +nightly test
  
# Run benchmarks
cd d01/a
cargo +nightly bench -- --nocapture
```

### Benchmarks

Benchmarks done with `cargo bench` on Intel i7-1185G7.

| Day                                          | a                              | b                              |
| -------------------------------------------- | ------------------------------ | ------------------------------ |
| [Day 1](https://adventofcode.com/2022/day/1) | [0.027 ms](./d01/a/src/lib.rs) | [0.029 ms](./d01/b/src/lib.rs) |
| [Day 2](https://adventofcode.com/2022/day/2) | [0.007 ms](./d02/a/src/lib.rs) | [0.005 ms](./d02/b/src/lib.rs) |
| [Day 3](https://adventofcode.com/2022/day/3) | [0.111 ms](./d03/a/src/lib.rs) | [0.016 ms](./d03/b/src/lib.rs) |
| [Day 4](https://adventofcode.com/2022/day/4) | [0.129 ms](./d04/a/src/lib.rs) | [0.058 ms](./d04/b/src/lib.rs) |
| [Day 5](https://adventofcode.com/2022/day/5) | [0.048 ms](./d05/a/src/lib.rs) | [0.056 ms](./d05/b/src/lib.rs) |
| [Day 6](https://adventofcode.com/2022/day/6) | [0.001 ms](./d06/a/src/lib.rs) | [0.244 ms](./d06/b/src/lib.rs) |
