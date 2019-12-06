use std::{collections::HashMap, fs};

fn get_input(fp: &str) -> Vec<Vec<String>> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .split('\n')
        .map(|x| x.split(')').map(String::from).collect())
        .collect()
}

fn get_tree(input: Vec<Vec<String>>) -> (HashMap<String, Vec<String>>, HashMap<String, String>) {
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    let mut parent: HashMap<String, String> = HashMap::new();
    for x in input.into_iter() {
        if tree.contains_key(&x[0]) {
            tree.get_mut(&x[0]).unwrap().push(x[1].clone());
        } else {
            tree.insert(x[0].clone(), vec![x[1].clone()]);
        }
        tree.entry(x[1].clone()).or_insert_with(|| vec![]);
        parent.insert(x[1].clone(), x[0].clone());
    }
    (tree, parent)
}

fn count(x: &str, current: usize, total: &mut usize, tree: &HashMap<String, Vec<String>>) -> usize {
    *total += current;
    for y in tree.get(x).unwrap().iter() {
        count(y, current+1, total, tree);
    }
    *total
}

fn get_parents(x: &str, parent: &HashMap<String, String>) -> Vec<String> {
    let mut parents = Vec::new();
    let mut y = parent.get(x).unwrap();
    while y != "COM" {
        parents.push(y.to_string());
        y = parent.get(y).unwrap();
    } 
    parents
}

fn part_one(tree: &HashMap<String, Vec<String>>) -> usize{
    count("COM", 0, &mut 0, tree)
}

fn part_two(parent: &HashMap<String, String>) -> usize {
    let parents_YOU = get_parents("YOU", parent);
    let parents_SAN = get_parents("SAN", parent);
    let mut common = "";
    for x in parents_YOU.iter() {
        if parents_SAN.contains(x) {
            common = x;
            break;
        }
    }
    parents_YOU.iter().position(|x| x==common).unwrap() + parents_SAN.iter().position(|x| x==common).unwrap()
}

fn main() {
    let input = get_input("../input.txt");
    let (tree, parent) = get_tree(input);
    println!("Part one: {}", part_one(&tree));
    println!("Part two: {}", part_two(&parent));
}
