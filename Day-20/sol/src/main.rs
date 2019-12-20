use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::FromIterator,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Node(usize, usize);

impl Node {
    fn neighs(&self) -> Vec<Self> {
        vec![
            Self(self.0 + 1, self.1),
            Self(self.0, self.1 - 1),
            Self(self.0 - 1, self.1),
            Self(self.0, self.1 + 1),
        ]
    }
}

fn get_input(fp: &str) -> Vec<String> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end_matches('\n')
        .split('\n')
        .map(|s| s.to_string())
        .collect()
}

fn parse(inp: &[String]) -> (Vec<Node>, HashMap<String, Vec<Node>>) {
    let mut floor = Vec::new();
    let mut gates: HashMap<String, Vec<Node>> = HashMap::new();
    let (n, m) = (inp.len(), inp[0].len());
    for i in 0..n {
        for j in 0..m {
            let c = inp[i].chars().nth(j).unwrap();
            if c == '.' {
                floor.push(Node(i, j));
            } else if c.is_ascii_uppercase() {
                let cip1 = inp
                    .get(i + 1)
                    .unwrap_or(&String::new())
                    .chars()
                    .nth(j)
                    .unwrap_or('@');
                let cjp1 = inp
                    .get(i)
                    .unwrap_or(&String::new())
                    .chars()
                    .nth(j + 1)
                    .unwrap_or('@');
                if i < n - 1 && cip1.is_ascii_uppercase() {
                    let t = if i > 1 && inp[i - 1].chars().nth(j).unwrap() == '.' {
                        Node(i - 1, j)
                    } else {
                        Node(i + 2, j)
                    };
                    let g = gates
                        .entry(String::from_iter([c, cip1].iter()))
                        .or_default();
                    g.push(t);
                } else if j < m - 1 && cjp1.is_ascii_uppercase() {
                    let t = if j > 1 && inp[i].chars().nth(j - 1).unwrap() == '.' {
                        Node(i, j - 1)
                    } else {
                        Node(i, j + 2)
                    };
                    let g = gates
                        .entry(String::from_iter([c, cjp1].iter()))
                        .or_default();
                    g.push(t);
                }
            }
        }
    }
    (floor, gates)
}

fn make_graph(floor: &[Node], gates: &HashMap<String, Vec<Node>>) -> HashMap<Node, Vec<Node>> {
    let mut graph = HashMap::new();
    for &n in floor.iter() {
        let mut neigh = Vec::new();
        for &x in n.neighs().iter().filter(|x| floor.contains(x)) {
            neigh.push(x);
        }
        for g in gates.values() {
            if g.len() == 2 && g.contains(&n) {
                neigh.push(g[(1 + g.iter().position(|&x| x == n).unwrap()) % 2]);
            }
        }
        graph.insert(n, neigh);
    }
    graph
}

fn bfs(graph: &HashMap<Node, Vec<Node>>, start: Node, end: Node) -> usize {
    let mut queue = vec![(start, 0)];
    let mut seen = HashSet::new();
    seen.insert(start);
    while !queue.is_empty() {
        let v = queue.remove(0);
        if v.0 == end {
            return v.1;
        }
        for &n in graph.get(&v.0).unwrap().iter() {
            if !seen.contains(&n) {
                queue.push((n, v.1 + 1));
                seen.insert(n);
            }
        }
    }
    0
}

type NodeD = (Node, isize);

fn make_graph2(
    floor: &[Node],
    gates: &HashMap<String, Vec<Node>>,
    h: usize,
    w: usize,
) -> HashMap<Node, Vec<NodeD>> {
    let mut graph = HashMap::new();
    for &n in floor.iter() {
        let mut neigh = Vec::new();
        for &x in n.neighs().iter().filter(|x| floor.contains(x)) {
            neigh.push((x, 0));
        }
        for g in gates.values() {
            if g.len() == 2 && g.contains(&n) {
                if n.0 == 2 || n.0 == h - 3 || n.1 == 2 || n.1 == w - 3 {
                    neigh.push((g[(1 + g.iter().position(|&x| x == n).unwrap()) % 2], -1));
                } else {
                    neigh.push((g[(1 + g.iter().position(|&x| x == n).unwrap()) % 2], 1));
                }
            }
        }
        graph.insert(n, neigh);
    }
    graph
}

fn bfs2(graph: &HashMap<Node, Vec<NodeD>>, start: Node, end: Node) -> usize {
    let mut queue = vec![(start, 0isize, 0)];
    let mut seen = HashSet::new();
    seen.insert((start, 0));
    while !queue.is_empty() {
        let v = queue.remove(0);
        if v.0 == end && v.1 == 0 {
            return v.2;
        }
        for &n in graph.get(&v.0).unwrap().iter() {
            if v.1 + n.1 < 0 {
                continue;
            }
            if !seen.contains(&(n.0, v.1 + n.1)) {
                queue.push((n.0, v.1 + n.1, v.2 + 1));
                seen.insert((n.0, v.1 + n.1));
            }
        }
    }
    0
}

fn part_one(inp: &[String]) -> usize {
    let (floor, gates) = parse(inp);
    let graph = make_graph(&floor, &gates);
    bfs(
        &graph,
        gates.get("AA").unwrap()[0],
        gates.get("ZZ").unwrap()[0],
    )
}
fn part_two(inp: &[String]) -> usize {
    let (floor, gates) = parse(inp);
    let graph = make_graph2(&floor, &gates, inp.len(), inp[0].len());
    bfs2(
        &graph,
        gates.get("AA").unwrap()[0],
        gates.get("ZZ").unwrap()[0],
    )
}

fn main() {
    let inp = get_input("../input.txt");
    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}