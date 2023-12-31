use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn read_inputs(filename: String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

#[derive(Debug, PartialEq, Eq)]
struct Galaxy {
    i: isize,
    y: isize,
    x: isize,
}

impl Galaxy {
    fn new(i: isize, y: isize, x: isize) -> Galaxy {
        Galaxy { i: i, y: y, x: x }
    }
}
fn main() {
    // For star1, replace with 2
    let mul = 1000000;
    let files = vec!["test", "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) => input = inputs,
            Err(_) => process::exit(0),
        }
        let input: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let mut galaxies = vec![];
        let mut i = 1;
        for ri in 0..input.len() {
            for ci in 0..input[ri].len() {
                if input[ri][ci] {
                    galaxies.push(Galaxy::new(i, ri as isize, ci as isize));
                    i += 1;
                }
            }
        }

        let mut offset = 0;
        let mut last = -1;
        for g in &mut galaxies {
            if g.y == last {
                g.y += offset * (mul - 1);
                continue;
            }
            offset += g.y - (last + 1);
            last = g.y;
            g.y += offset * (mul - 1);
        }
        galaxies.sort_by_key(|g| g.x);
        offset = 0;
        last = -1;
        for g in &mut galaxies {
            if g.x == last {
                g.x += offset * (mul - 1);
                continue;
            }
            offset += g.x - (last + 1);
            last = g.x;
            g.x += offset * (mul - 1);
        }

        galaxies.sort_by_key(|g| g.i);
        let mut sum = 0;
        for i1 in 0..galaxies.len() {
            for i2 in (i1 + 1)..galaxies.len() {
                let g1 = &galaxies[i1];
                let g2 = &galaxies[i2];
                //println!(
                //    "{} - {} : {}",
                //    i1 + 1,
                //    i2 + 1,
                //    num::abs(g1.x - g2.x) + num::abs(g1.y - g2.y)
                //);
                sum += num::abs(g1.x - g2.x) + num::abs(g1.y - g2.y);
            }
        }
        println!("star2: {}", sum);
    }
}
