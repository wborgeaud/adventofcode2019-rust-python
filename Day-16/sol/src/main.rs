use std::fs;

fn get_input(fp: &str) -> Vec<isize> {
    fs::read_to_string(fp)
        .expect("Cannot read file")
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

fn fft(a: Vec<isize>) -> Vec<isize> {
    let patt: [isize; 4] = [0, 1, 0, -1];
    let n = a.len();
    let mut new = Vec::new();
    for i in 1..=n {
        let mut res = 0isize;
        for j in 1..=n {
            res += a[j - 1] * patt[(j / i) % 4];
        }
        new.push(res.abs() % 10);
    }
    new
}

fn fast_fft(a: &[isize], offset: usize, rep: usize) -> String {
    let mut b = a[offset..].to_vec();
    for _ in 0..rep {
        let mut new = Vec::new();
        let mut res = 0;
        for x in b.iter().rev() {
            res += x;
            new.push(res.abs() % 10);
        }
        new.reverse();
        b = new;
    }
    b[..8]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn part_one(inp: &[isize]) -> String {
    let mut x = inp.to_vec();
    for _ in 0..100 {
        x = fft(x);
    }
    x[..8]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn part_two(inp: &[isize]) -> String {
    let offset: usize = inp[..7]
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    let mut a = Vec::new();
    for _ in 0..10000 {
        a.extend(inp.iter());
    }
    fast_fft(&a, offset, 100)
}

fn main() {
    let inp = get_input("../input.txt");
    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
