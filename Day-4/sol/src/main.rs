fn is_good_one(n: usize) -> bool {
    let sn = n.to_string();
    (
        sn.len() == 6 &&
        (1..sn.len()).any(|i| sn.chars().nth(i)==sn.chars().nth(i-1)) &&
        (1..sn.len()).all(|i| sn.chars().nth(i)>=sn.chars().nth(i-1))
    )
}

fn matches(s: &str) -> Vec<usize> {
    let mut res = Vec::new();
    let mut chars = s.chars();
    let (mut i, mut j) = (0, 1);
    loop {
        if s.chars().nth(j) == s.chars().nth(i) {
            j += 1;
        } else {
            res.push(j-i);
            if j >= s.len() {
                return res;
            }
            i = j;
            j = i + 1;
        }
    }
    res
}

fn is_good_two(n: usize) -> bool {
    let sn = n.to_string();
    (
        sn.len() == 6 &&
        (1..sn.len()).any(|i| sn.chars().nth(i)==sn.chars().nth(i-1)) &&
        (1..sn.len()).all(|i| sn.chars().nth(i)>=sn.chars().nth(i-1)) &&
        matches(&sn).iter().any(|&i| i==2)
    )
}

fn part_one(a: usize, b: usize) -> usize {
    (a..b+1).filter(|&i| is_good_one(i)).count()
}

fn part_two(a: usize, b: usize) -> usize {
    (a..b+1).filter(|&i| is_good_two(i)).count()
}

fn main() {
    let inp = (206938, 679128);
    println!("Part one: {}", part_one(inp.0, inp.1));
    println!("Part two: {}", part_two(inp.0, inp.1));
}
