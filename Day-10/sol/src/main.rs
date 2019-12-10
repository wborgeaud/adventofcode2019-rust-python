use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
struct Point(isize, isize);

impl Point {
    fn colinear(&self, y: &Self, z: &Self) -> bool {
        ((self.0 - z.0) * (y.0 - z.0) + (self.1 - z.1) * (y.1 - z.1)) >= 0
            && ((self.0 - z.0) * (y.1 - z.1) - (self.1 - z.1) * (y.0 - z.0)) == 0
    }
    fn angle(&self, z: &Self) -> f64 {
        -((self.0 - z.0) as f64).atan2((self.1 - z.1) as f64)
    }
    fn norm(&self, z: &Self) -> isize {
        (self.0 - z.0) * (self.0 - z.0) + (self.1 - z.1) * (self.1 - z.1)
    }
}

fn get_input(fp: &str) -> Vec<Point> {
    let m: Vec<Vec<bool>> = fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .split('\n')
        .map(|x| x.chars().map(|c| c == '#').collect())
        .collect();
    let mut res = Vec::new();
    for i in 0..m.len() {
        res.extend(
            (0..m[0].len())
                .filter(|&j| m[i][j])
                .map(|j| Point(j as isize, i as isize)),
        );
    }
    res
}

fn part_one(map: &[Point]) -> (Point, usize) {
    let mut sight: HashMap<&Point, Vec<&Point>> = HashMap::new();
    for a in map.iter() {
        sight.insert(a, Vec::new());
        for k in map.iter() {
            if k == a {
                continue;
            }
            if !sight.get(a).unwrap().iter().any(|s| s.colinear(k, a)) {
                sight.get_mut(a).unwrap().push(k);
            }
        }
    }
    let mut max = 0;
    let mut best = Point(0, 0);
    sight.iter().for_each(|(k, v)| {
        if v.len() > max {
            max = v.len();
            best = **k;
        }
    });
    (best, max)
}

fn polar_coords(map: &[Point], x: Point) -> Vec<(f64, isize, &Point)> {
    let mut pol: Vec<(f64, isize, &Point)> = Vec::new();
    for a in map.iter() {
        if *a == x {
            continue;
        }
        pol.push((a.angle(&x), a.norm(&x), a));
    }
    pol.sort_by(|x, y| (x.0, x.1).partial_cmp(&(y.0, y.1)).unwrap());
    pol
}

fn part_two(map: &[Point], x: Point) -> isize {
    let mut pol = polar_coords(map, x);
    let mut i = 0;
    loop {
        let mut angle = -100.0;
        let mut seen = Vec::new();
        for (ind, x) in pol.iter().enumerate() {
            if (x.0 - angle).abs() < 1e-5 {
                continue;
            }
            seen.push(ind);
            angle = x.0;
            i += 1;
            if i == 200 {
                return (x.2).0 * 100 + (x.2).1;
            }
        }
        pol = (0..pol.len())
            .filter(|i| !seen.contains(i))
            .map(|i| pol[i])
            .collect();
    }
}

fn main() {
    let map = get_input("../input.txt");
    let (x, m) = part_one(&map);
    println!("Part one: {}", m);
    println!("Part two: {}", part_two(&map, x));
}
