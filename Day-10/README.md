# Day 10

Counting and sorting points in polar coordinates. 

## Python

67 lines of code.

```bash
$ time python sol.py
Part one: 296
Part two: 204

real	0m14.812s
user	0m14.800s
sys	0m0.000s
```



## Rust

103 lines of code.

```bash
$ time ./target/release/sol
Part one: 296
Part two: 204

real	0m0.089s
user	0m0.080s
sys	0m0.004s
```



## Development differences

The Rust solution is quite dirty, mainly due to the fact that `f64` doesn't implement `Hash`, so I had to use a dirty trick in sorting the asteroids in polar coordinates.

On the other hand, it is ~200 times faster than the Python solution, which shows that any solution involving loops is very slow in Python.  