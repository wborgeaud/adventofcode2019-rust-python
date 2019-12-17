# Day 14

Find the smallest quantity satisfying given restrictions. (**WARNING:** Very ugly code)

## Python

102 lines of code.

```bash
$ time python sol.py
Part one: 892207
Part two: 1935265

real	0m0.243s
user	0m0.288s
sys	0m0.208s
```



## Rust

135 lines of code.

```bash
$ time ./target/release/sol
Part one: 892207
Part two: 1935265

real	0m0.103s
user	0m0.092s
sys	0m0.008s
```

## Development differences

Rust is only 2x faster than Python for this solution. Will need to investigate further to find out why.
