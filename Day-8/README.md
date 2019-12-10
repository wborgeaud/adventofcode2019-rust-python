# Day 8

Building a bitmap image.

## Python

33 lines of code.

```bash
$ time python sol.py
Part one: 2286
Part two: 
 ⬛⬛    ⬛⬛ ⬛⬛⬛⬛ ⬛    ⬛⬛⬛  
⬛  ⬛    ⬛    ⬛ ⬛    ⬛  ⬛ 
⬛       ⬛   ⬛  ⬛    ⬛  ⬛ 
⬛       ⬛  ⬛   ⬛    ⬛⬛⬛  
⬛  ⬛ ⬛  ⬛ ⬛    ⬛    ⬛    
 ⬛⬛   ⬛⬛  ⬛⬛⬛⬛ ⬛⬛⬛⬛ ⬛    


real	0m0.244s
user	0m0.284s
sys	0m0.200s
```



## Rust

58 lines of code.

```bash
$ time ./target/release/sol
Part one: 2286
Part two: 
 ⬛⬛    ⬛⬛ ⬛⬛⬛⬛ ⬛    ⬛⬛⬛  
⬛  ⬛    ⬛    ⬛ ⬛    ⬛  ⬛ 
⬛       ⬛   ⬛  ⬛    ⬛  ⬛ 
⬛       ⬛  ⬛   ⬛    ⬛⬛⬛  
⬛  ⬛ ⬛  ⬛ ⬛    ⬛    ⬛    
 ⬛⬛   ⬛⬛  ⬛⬛⬛⬛ ⬛⬛⬛⬛ ⬛    


real	0m0.010s
user	0m0.004s
sys	0m0.004s
```



## Development differences

I used `numpy` in Python and `ndarray` in Rust. As a long time `numpy` user, the Python solution seems very natural. It was the first time that I used `ndarray` and the API feels a bit off to me and it took me a while to get the solution right. I hope I won't have to use more `ndarray` for the rest of the challenges... 

