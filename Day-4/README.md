# Day 4

Counting numbers satisfying conditions on their digits.

## Python

29 lines of code. Using regex backreferences for part two.

```bash
$ time python sol.py
Part one: 1653
Part two: 1133

real	0m2.767s
user	0m2.744s
sys	0m0.020s
```



## Rust

51 lines of code. Using custom enumeration of matches for part two.

```bash
$ time ./target/release/sol
Part one: 1653
Part two: 1133

real	0m0.134s
user	0m0.128s
sys	0m0.004s
```



## Development differences

After observing the speed difference between Rust and Python, I wondered if it came from regex inefficiency. I thus implemented the matching algorithm from Rust to Python in the file `sol2.py`. 
```bash
$ time python sol2.py
Part one: 1653
Part two: 1133

real	0m2.441s
user	0m2.428s
sys	0m0.008s
```

It gives slightly better result than the regex implementation (at the cost of 13 LoC) but is still way behind Rust. I guess string manipulations and for loops are very slow in Python...
