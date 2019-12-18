use std::{fs, collections::{HashMap, HashSet},iter::FromIterator};
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position(usize, usize);

impl Position {
    fn neighs(&self, h: usize, w: usize) -> Vec<Self> {
        vec![Self(self.0+1, self.1), Self(self.0, self.1-1), Self(self.0-1, self.1), Self(self.0, self.1+1)]
    }
}

#[derive(Debug, Clone, Copy)]
enum Cave {
    Floor,
    Key(char),
    Door(char)
}

type Map = HashMap<Position, Cave>;
type KeysL = HashMap<char, Position>;
type Paths = HashMap<(char, char), (Vec<char>, usize)>;

fn get_input(fp: &str) -> Vec<String> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn get_map(inp: &[String]) -> (Map, Position, KeysL) {
    let mut map = Map::new();
    let mut keys = KeysL::new();
    let mut start = Position(0,0);
    for i in 0..inp.len() {
        for j in 0..inp[0].len() {
            let c = inp[i].chars().nth(j).unwrap();
            if c == '@' {
                start = Position(i,j);
            } else if c == '#' {
                continue;
            } else if c.is_ascii_lowercase() {
                keys.insert(c, Position(i,j));
                map.insert(Position(i,j), Cave::Key(c));
            } else if c.is_ascii_uppercase() {
                map.insert(Position(i,j), Cave::Door(c));
            } else if c == '.' {
                map.insert(Position(i,j), Cave::Floor);
            }
        }
    }
    (map, start, keys)
}

fn find_path(s: Position, t: Position, map: &Map, h: usize, w: usize) -> Option<(Position, Vec<char>, usize)> {
    let mut queue: Vec<(Position, Vec<char>, usize)> = vec![(s, vec![], 0)];
    let mut seen = HashSet::new();
    seen.insert(s);
    while !queue.is_empty() {
        let v = queue.remove(0);
        if v.0 == t {
            return Some(v);
        }
        for n in v.0.neighs(h, w).iter() {
            if map.contains_key(n) && !seen.contains(n) {
                if let Cave::Door(x) = map.get(n).unwrap() {
                    let mut nv = v.1.clone();
                    nv.push(*x);
                    queue.push((*n, nv, v.2+1));
                } else {
                    queue.push((*n, v.1.clone(), v.2+1));
                }
                seen.insert(*n);
            }
        } 
    }
    None
}

fn solve(keys: &KeysL, paths: &Paths) -> Option<Vec<char>> {
    let keys_list: Vec<_> = keys.keys().collect();
    let mut queue = vec![(vec!['@'],0)];
    while !queue.is_empty() {
        let v = queue.remove(0);
        if v.0.len() == keys_list.len()+1 {
            return Some(v.0);
        }
        'letters: loop {
            let l = keys_list.choose(&mut rand::thread_rng()).unwrap(); 
            if v.0.contains(l) {
                continue 'letters;
            }
            let p = paths.get(&(*v.0.last().unwrap(), **l)).unwrap();
            if p.0.iter().all(|c| v.0.contains(&c.to_ascii_lowercase())) {
                let mut nv = v.0.clone();
                nv.push(**l);
                queue.push((nv, v.1+p.1));
                break 'letters;
            }
        }
    }
    None
}

fn cost(path: &[char], paths: &Paths) -> usize {
    let mut old = '@';
    let mut ans = 0;
    for i in 1..path.len() {
        let p = paths.get(&(old, path[i])).unwrap();
        if p.0.iter().all(|c| path[..i].contains(&c.to_ascii_lowercase())) {
            ans += p.1;
        } else {
            return 10000;
        }
        old = path[i];
    }
    ans
}

fn better(path: &[char], paths: &Paths) -> Vec<char> {
    let mut best = path.to_vec();
    let mut min = cost(path, paths);
    for i in 1..path.len() {
        for j in i+1..path.len() {
            for k in j+1..path.len() {
                let mut l = path.to_vec();
                let olds = (l[i], l[j], l[k]);
                l[i] = olds.1;
                l[j] = olds.2;
                l[k] = olds.0;
                let c = cost(&l, paths);
                if c < min {
                    best = l;
                    min = c;
                }
                let mut l = path.to_vec();
                let olds = (l[i], l[j], l[k]);
                l[i] = olds.2;
                l[j] = olds.0;
                l[k] = olds.1;
                let c = cost(&l, paths);
                if c < min {
                    best = l;
                    min = c;
                }
            }
        }
    }
    best
}

fn part_one(inp: &[String]) -> usize {
    let (h, w) = (inp.len(), inp[0].len());
    let (map, start, keys) = get_map(inp);
    let mut paths = Paths::new();
    for l in keys.keys() {
        for ll in keys.keys() {
            if l == ll {continue;}
            let p = find_path(*keys.get(l).unwrap(), *keys.get(ll).unwrap(), &map, h, w).unwrap();
            paths.insert((*l, *ll), (p.1, p.2));
        }
        let p = find_path(start, *keys.get(l).unwrap(), &map, h, w).unwrap();
        paths.insert(('@', *l), (p.1, p.2));
    }
    let mut min = 10000;
    for _ in 0..10 {
        let mut path = solve(&keys, &paths).unwrap();
        let mut c = cost(&path, &paths);
        loop {
            path = better(&path, &paths);
            let nc = cost(&path, &paths);
            if nc==c {break;}
            else {c=nc;}
            if c < min {min=c;}
        }
    }
    min
}

fn solve2(keys: &KeysL, paths: &Paths, categories: &HashMap<char, usize>) -> Option<Vec<(usize, char)>> {
    let keys_list: Vec<_> = keys.keys().collect();
    let mut queue = vec![(vec![vec!['0'],vec!['1'],vec!['2'],vec!['3']],0, vec![])];
    while !queue.is_empty() {
        let v = queue.remove(0);
        if v.0[0].len()+v.0[1].len()+v.0[2].len()+v.0[3].len() == keys_list.len()+4 {
            return Some(v.2);
        }
        'letters: loop {
            let l = keys_list.choose(&mut rand::thread_rng()).unwrap(); 
            let i = *categories.get(l).unwrap();
            if v.0[i].contains(l) {
                continue 'letters;
            }
            let p = paths.get(&(*v.0[i].last().unwrap(), **l)).unwrap();
            if p.0.iter().all(|c| v.0[0].contains(&c.to_ascii_lowercase()) ||
                                  v.0[1].contains(&c.to_ascii_lowercase()) ||  
                                  v.0[2].contains(&c.to_ascii_lowercase()) ||  
                                  v.0[3].contains(&c.to_ascii_lowercase()) ) {
                let mut nv = v.0.clone();
                nv[i].push(**l);
                let mut nv2 = v.2.clone();
                nv2.push((i, **l));
                queue.push((nv, v.1+p.1, nv2));
                break 'letters;
            }
        }
    }
    None
}

fn cost2(path: &[(usize, char)], paths: &Paths) -> usize {
    let mut olds = ['0','1','2','3'];
    let mut ans = 0;
    for it in 0..path.len() {
        let (i, l) = path[it];
        let p = paths.get(&(olds[i], l)).unwrap();
        if p.0.iter().all(|c| path[..it].iter().map(|a| a.1).any(|x| x==c.to_ascii_lowercase())) {
            ans += p.1;
            // println!("{} {} {}", olds[i], l, p.1);
        } else {
            return 10000;
        }
        olds[i] = l;
    }
    ans
}

fn better2(path: &[(usize,char)], paths: &Paths) -> (Vec<(usize,char)>, usize) {
    let mut best = path.to_vec();
    let mut min = cost2(path, paths);
    for i in 1..path.len() {
        for j in i+1..path.len() {
            for k in j+1..path.len() {
                let mut l = path.to_vec();
                let olds = (l[i], l[j], l[k]);
                l[i] = olds.1;
                l[j] = olds.2;
                l[k] = olds.0;
                let c = cost2(&l, paths);
                if c < min {
                    best = l;
                    min = c;
                }
                let mut l = path.to_vec();
                let olds = (l[i], l[j], l[k]);
                l[i] = olds.2;
                l[j] = olds.0;
                l[k] = olds.1;
                let c = cost2(&l, paths);
                if c < min {
                    best = l;
                    min = c;
                }
            }
        }
    }
    (best, min) 
}

fn part_two(inp: &[String]) -> usize {
    let (h, w) = (inp.len(), inp[0].len());
    let (mut map, start, keys) = get_map(inp);
    let starts = [Position(start.0+1, start.1+1), Position(start.0-1, start.1+1), Position(start.0+1, start.1-1), Position(start.0-1, start.1-1)];
    map.remove(&start);
    for x in start.neighs(h, w).iter() {
        map.remove(x);
    }
    let mut paths = Paths::new();
    for l in keys.keys() {
        for ll in keys.keys() {
            if l == ll {continue;}
            let p_op = find_path(*keys.get(l).unwrap(), *keys.get(ll).unwrap(), &map, h, w);
            if let Some(p) = p_op {
                paths.insert((*l, *ll), (p.1, p.2));
            }
        }
        for it in 0..4 {
            let p_op = find_path(starts[it], *keys.get(l).unwrap(), &map, h, w);
            if let Some(p) = p_op {
                paths.insert((std::char::from_digit(it as u32, 10).unwrap(), *l), (p.1, p.2));
            }
        }
    }
    let mut categories: HashMap<char, usize> = HashMap::new();
    for l in keys.keys() {
        let j = (0..4).find(|i| paths.contains_key(&(std::char::from_digit(*i as u32, 10).unwrap(), *l)));
        categories.insert(*l, j.unwrap());
    }
    let mut min = 10000;
    for _ in 0..100 {
        let path = solve2(&keys, &paths, &categories).unwrap();
        let mut c = cost2(&path, &paths);
        loop {
            let (_path, nc) = better2(&path, &paths);
            if nc==c {break;}
            else {c=nc;}
            if c < min {min=c;}
        }
    }
    min
}

fn main() {
    let inp = get_input("../input.txt");
    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
