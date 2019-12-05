# Day 5

Getting serious with the Intcode machine from day 2.

## Python

159 lines of code.

```bash
$ time python sol.py
Running part one
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 11933517
Running part two
Output: 10428568

real	0m0.046s
user	0m0.032s
sys	0m0.016s
```



## Rust

214 lines of code.

```bash
$ time ./target/release/sol
Running part one
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 0
Output: 11933517
Running part two
Output: 10428568

real	0m0.006s
user	0m0.004s
sys	0m0.000s
```



## Development differences

To distinguish all the opcodes, Rust's `match` feels much more natural and elegant than Python's `elif`.
