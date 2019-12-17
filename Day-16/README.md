# Day 10

Finding cycles in a 4-body problem dynamical system. 

## Python

49 lines of code.

```bash
$ time python sol.py
Part one: 13500
Part two: 278013787106916

real    0m4.295s
user    0m4.316s
sys 0m0.284s
```



## Rust

135 lines of code.

```bash
$ time ./target/release/sol
Part one: 13500
Part two: 278013787106916

real	0m0.054s
user	0m0.048s
sys	0m0.004s
```

## Development differences

I used `numpy` in Python which makes the solution quite elegant. I didn't want to use `ndarray` in Rust, so the solution is much longer but still 100 times faster. Again, loops are killing us in Python...
