use regex::Regex;
use std::{collections::HashMap, fs, iter::FromIterator};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Chemical(String);
type Reactions = HashMap<(isize, Chemical), Vec<(isize, Chemical)>>;

fn parse(s: &str, reacs: &mut Reactions) {
    let re = Regex::new(r"(\d+)[,\s]*(\w+)").unwrap();
    let mut v: Vec<(isize, Chemical)> = re
        .captures_iter(s)
        .map(|c| (c[1].parse().unwrap(), Chemical(c[2].to_string())))
        .collect();
    let f = v.pop().unwrap();
    reacs.insert(f, v);
}

fn get_input(fp: &str) -> Reactions {
    let mut reactions: Reactions = HashMap::new();
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .split('\n')
        .for_each(|s| parse(s, &mut reactions));
    reactions
}

fn get_ore_reac(reactions: &Reactions) -> Vec<Chemical> {
    reactions
        .iter()
        .filter(|(k, v)| v.len() == 1 && v[0].1 == Chemical("ORE".to_string()))
        .map(|(k, v)| k.1.clone())
        .collect()
}

fn find_reac(
    reactions: &Reactions,
    chem: &Chemical,
) -> ((isize, Chemical), Vec<(isize, Chemical)>) {
    reactions
        .iter()
        .find(|(k, v)| k.1 == *chem)
        .map(|(k, v)| (k.clone(), v.clone()))
        .unwrap()
}

fn ore_price(ore_buy: &HashMap<Chemical, isize>, reactions: &Reactions) -> isize {
    let mut ans = 0;
    for (x,v) in ore_buy.iter() {
        let (r, orr) = find_reac(reactions, x);
        let num = ((*v as f64) / (r.0 as f64)).ceil() as isize;
        ans += num*orr[0].0;
    }
    ans
}

fn fuel_price(reactions: &Reactions, fuel_quantity: isize) -> isize {
    let ore_reac = get_ore_reac(reactions);
    let mut reacs = HashMap::new();
    reacs.insert(Chemical("FUEL".to_string()), fuel_quantity);
    let mut buy = HashMap::<Chemical, Vec<isize>>::from_iter(
        reactions.iter().map(|(k, v)| (k.1.clone(), Vec::new())),
    );
    while !reacs.is_empty() {
        let mut next_reacs: HashMap<Chemical, isize> = HashMap::new();
        for r in reacs.keys() {
            if !ore_reac.contains(r) {
                buy.get_mut(r).unwrap().push(*reacs.get(r).unwrap());
                let (x, new_reac) = find_reac(&reactions, r);
                let num: isize = ((*reacs.get(r).unwrap() as f64) / (x.0 as f64)).ceil() as isize;
                for (i, rr) in new_reac.into_iter() {
                    let bam = next_reacs.entry(rr).or_insert(0);
                    *bam += num * i;
                }
            }
        }
        reacs = next_reacs;
    }
    let mut needs: HashMap<Chemical, isize> = HashMap::new();
    buy.into_iter().for_each(|(x, v)| {
        needs.insert(x, v.iter().sum());
    });
    loop {
        let mut reacs = HashMap::new();
        reacs.insert(Chemical("FUEL".to_string()), fuel_quantity);
        let mut ore_buy =
            HashMap::<Chemical, isize>::from_iter(ore_reac.iter().map(|c| (c.clone(), 0)));
        let mut stocks =
            HashMap::<Chemical, isize>::from_iter(reactions.iter().map(|(k, _)| (k.1.clone(), 0)));
        while !reacs.is_empty() {
            let mut next_reacs: HashMap<Chemical, isize> = HashMap::new();
            for r in reacs.keys() {
                if ore_reac.contains(r) {
                    ore_buy.entry(r.clone()).and_modify(|x| *x+=reacs.get(r).unwrap());
                } else if stocks.contains_key(r) && reacs.get(r)<=stocks.get(r) {
                    stocks.entry(r.clone()).and_modify(|x| *x-=reacs.get(r).unwrap());
                } else {
                    let (x, new_reac) = find_reac(&reactions, r);
                    let num: isize =
                        ((*needs.get(r).unwrap() as f64) / (x.0 as f64)).ceil() as isize;
                    for (i, rr) in new_reac.into_iter() {
                        let bam = next_reacs.entry(rr).or_insert(0);
                        *bam += num * i;
                    }
                    stocks.entry(r.clone()).and_modify(|x| *x+=(needs.get(r).unwrap()-reacs.get(r).unwrap()));
                }
            }
            reacs = next_reacs;
        }
        if stocks.values().all(|&v| v==0) {
            return ore_price(&ore_buy, reactions);
        } else {
            needs.iter_mut().for_each(|(k,v)| {
                *v -= stocks.get(k).unwrap();
            });
        }
    }
}

fn part_one(reactions: &Reactions) -> isize {
    fuel_price(reactions, 1)
}

fn part_two(reactions: &Reactions) -> isize {
    let trillion = 1_000_000_000_000;
    let (mut a, mut b) = (0, 100*trillion/part_one(reactions));
    loop {
        let m = (a+b)/2;
        let x = fuel_price(reactions, m);
        if x == trillion || m == a || m == b {
            return m;
        } else if x > trillion {
            b = m;
        } else {
            a = m;
        }
    }
}

fn main() {
    let reactions = get_input("../input.txt");
    println!("Part one: {}", part_one(&reactions));
    println!("Part two: {}", part_two(&reactions));
}