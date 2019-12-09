# Day 7

Running several Intcode machines concurrently.

## Python

58 LoCs for `sol.py` and 151 LoCs for `computer.py`.

```bash
$ time python sol.py
Part one: 277328
Part two: 11304734

real	0m0.399s
user	0m0.320s
sys	0m0.100s
```



## Rust

85 LoCs for `main.rs` and 192 LoCs for `computer.rs`.

```bash
$ time ./target/release/sol
Part one: 277328
Part one: 11304734

real	0m0.101s
user	0m0.044s
sys	0m0.052s
```



## Development differences

First hard challenge for me since I decided to use mutlithreading for part two. The `channel` API of Rust feels safer and easier than the `Queue` API of Python to pass data between concurrent Intcode computers. However, the Rust compiler makes it hard to keep a reference of each computers, making it difficult to know whether the computation is finished. I used the dirty trick of checking if the input channel was open, but that's far from optimal... Python is much more lax about this and the code is cleaner in my opinion.  
