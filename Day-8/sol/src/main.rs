use std::fs;
use ndarray::prelude::*;

fn get_input(fp: &str) -> Vec<usize> {
        fs::read_to_string(fp)
            .expect("Cannot read file")
            .trim_end()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
}

fn into_layers(inp: &[usize], h: usize, w: usize) -> Array3<usize> {
    let depth = inp.len()/(h*w);
    let mut layers: Array3<usize> = Array3::zeros((h,w,depth));
    let a = h*w;
    for (i, s) in inp.iter().enumerate() {
        layers[[(i%a)/w, (i%a)%w, i/a]] = *s;
    }
    layers
}

fn part_one(layers: &Array3<usize>) -> usize {
    let depth = layers.shape()[2];
    let num_zeros: Vec<usize> = (0..depth).map(|i| {
        let a = layers.index_axis(Axis(2), i);
        a.iter().filter(|&x| *x==0).count()
    }).collect();
    let mut min_zeros: Vec<usize> = (0..depth).collect();
    min_zeros.sort_by(|i,j| num_zeros[*i].partial_cmp(&num_zeros[*j]).unwrap());
    let a = layers.index_axis(Axis(2), min_zeros[0]);
    a.iter().filter(|&x| *x==1).count() * a.iter().filter(|&x| *x==2).count() 
}

fn part_two(layers: &Array3<usize>) -> String {
    let (height, width) = (6, 25);
    let mut image = String::from("\n");
    for i in 0..height {
        for j in 0..width {
            let ll = layers.slice(s![i, j, ..]);
            let x = ll.iter().find(|&&y| y != 2).unwrap();
            if *x == 1 {
                image.push('⬛');
            } else {
                image.push(' ');
            }
        }
        image.push('\n');
    }
    image
}

fn main() {
    let inp = get_input("../input.txt");
    let layers = into_layers(&inp, 6, 25);
    println!("Part one: {}", part_one(&layers));
    println!("Part two: {}", part_two(&layers));
}
