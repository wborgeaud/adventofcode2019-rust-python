# Day 11

Another easy Intcode challenge, this time with spacial instructions.

## Python

47 LoCs for `sol.py` and 129 LoCs for `computer.py`.

```bash
$ time python sol.py
Part one: 2219
Part two: 
⬛  ⬛  ⬛⬛  ⬛⬛⬛⬛ ⬛  ⬛ ⬛     ⬛⬛  ⬛⬛⬛  ⬛⬛⬛⬛
⬛  ⬛ ⬛  ⬛ ⬛    ⬛  ⬛ ⬛    ⬛  ⬛ ⬛  ⬛ ⬛   
⬛⬛⬛⬛ ⬛  ⬛ ⬛⬛⬛  ⬛  ⬛ ⬛    ⬛  ⬛ ⬛  ⬛ ⬛⬛⬛ 
⬛  ⬛ ⬛⬛⬛⬛ ⬛    ⬛  ⬛ ⬛    ⬛⬛⬛⬛ ⬛⬛⬛  ⬛   
⬛  ⬛ ⬛  ⬛ ⬛    ⬛  ⬛ ⬛    ⬛  ⬛ ⬛    ⬛   
⬛  ⬛ ⬛  ⬛ ⬛     ⬛⬛  ⬛⬛⬛⬛ ⬛  ⬛ ⬛    ⬛⬛⬛⬛


real	0m0.706s
user	0m0.716s
sys	0m0.060s
```



## Rust

84 LoCs for `main.rs` and 174 LoCs for `computer.rs`.

```bash
$ time ./target/release/sol
Part one: 2219
Part two: 
 ⬛  ⬛  ⬛⬛  ⬛⬛⬛⬛ ⬛  ⬛ ⬛     ⬛⬛  ⬛⬛⬛  ⬛⬛⬛⬛   
 ⬛  ⬛ ⬛  ⬛ ⬛    ⬛  ⬛ ⬛    ⬛  ⬛ ⬛  ⬛ ⬛      
 ⬛⬛⬛⬛ ⬛  ⬛ ⬛⬛⬛  ⬛  ⬛ ⬛    ⬛  ⬛ ⬛  ⬛ ⬛⬛⬛    
 ⬛  ⬛ ⬛⬛⬛⬛ ⬛    ⬛  ⬛ ⬛    ⬛⬛⬛⬛ ⬛⬛⬛  ⬛      
 ⬛  ⬛ ⬛  ⬛ ⬛    ⬛  ⬛ ⬛    ⬛  ⬛ ⬛    ⬛      
 ⬛  ⬛ ⬛  ⬛ ⬛     ⬛⬛  ⬛⬛⬛⬛ ⬛  ⬛ ⬛    ⬛⬛⬛⬛   


real	0m0.100s
user	0m0.100s
sys	0m0.040s
```


