use std::collections::HashMap;
use std::fs;

fn get_input(fp: &str) -> Vec<Vec<(char, isize)>> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .split('\n')
        .map(|x| {
            x.split(',')
                .map(|y| (y.chars().nth(0).unwrap(), y[1..].parse().unwrap()))
                .collect()
        })
        .collect()
}

fn fill_points(instructions: &[(char, isize)]) -> HashMap<(isize, isize), isize> {
    let mut point: (isize, isize) = (0, 0);
    let mut points = HashMap::new();
    let mut step = 1;
    let mut npoint = point;
    for x in instructions.iter() {
        let direction: (isize, isize);
        match x.0 {
            'R' => direction = (0, 1),
            'L' => direction = (0, -1),
            'U' => direction = (1, 0),
            'D' => direction = (-1, 0),
            _ => continue,
        }
        for i in 1..=x.1 {
            npoint = (point.0 + i * direction.0, point.1 + i * direction.1);
            points.entry(npoint).or_insert(step);
            step += 1;
        }
        point = npoint;
    }
    points
}

fn part_one(
    intersection: &[(isize,isize)],
) -> isize {
    intersection
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min_by_key(|&x| x)
        .unwrap()
}
fn part_two(
    intersection: &[(isize,isize)],
    points1: &HashMap<(isize, isize), isize>,
    points2: &HashMap<(isize, isize), isize>,
) -> isize {
    intersection
        .iter()
        .map(|k| points1.get(k).unwrap() + points2.get(k).unwrap())
        .min_by_key(|&x| x)
        .unwrap()
}

fn main() {
    let input = get_input("../input.txt");
    let points1 = fill_points(&input[0]);
    let points2 = fill_points(&input[1]);
    let intersection: Vec<(isize, isize)> = points1
        .keys()
        .filter(|k| points2.contains_key(k))
        .copied()
        .collect();
    println!("Part one: {}", part_one(&intersection));
    println!("Part two: {:?}", part_two(&intersection,&points1, &points2));
}
