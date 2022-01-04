use std::fs::File;
use std::io::BufReader;
use std::process;
use std::cmp::{max, min};
use std::io::prelude::*;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}
const MAX_AMOUNT :usize = 150;
//const MAX_AMOUNT :usize = 150;
fn star1(containers : &Vec<usize>, pos : usize, amount : usize) -> usize {
    if amount == MAX_AMOUNT { return 1; }
    if amount > MAX_AMOUNT { return 0; }
    if pos == containers.len() { return 0; }

    return star1(containers, pos+1, amount + containers[pos]) + star1(containers, pos+1, amount);
}

fn star2(containers : &Vec<usize>, pos : usize, amount : usize, num_cont : usize, min_cont : &mut usize, num_ways : &mut usize){

    if amount == MAX_AMOUNT {
        if num_cont == *min_cont { *num_ways += 1; return; }
        if num_cont < *min_cont { *num_ways = 1; *min_cont = num_cont; return; }
    }
    if amount > MAX_AMOUNT || pos == containers.len() { return; }
    if num_cont > *min_cont { return; }

    star2(containers, pos+1, amount + containers[pos], num_cont + 1, min_cont, num_ways);
    star2(containers, pos+1, amount, num_cont, min_cont, num_ways);
}

fn main() {
    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>
            input = inputs,
        Err(_) => process::exit(0),
    }

    let mut containers : Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("star1 : {}", star1(&containers, 0, 0));

    let mut min_cont : usize = containers.len();
    let mut num_ways : usize = 0;
    star2(&containers, 0, 0, 0, &mut min_cont, &mut num_ways);
    println!("star2 : ({}) {}", min_cont, num_ways);
}
