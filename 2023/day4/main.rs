use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::collections::HashSet;
use std::cmp::min;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_num(s: &str) -> i32{
    return s.parse::<i32>().unwrap();
}

fn fill_set(cards : &str) -> HashSet<i32> {

    let mut nums :HashSet<i32> = HashSet::new();
    let split: Vec<&str>= cards.split(" ").collect();
    for n in split {
        if n.len() > 0 {
            nums.insert(get_num(n));
        }
    }
    return nums;
}

fn get_num_winning(line : &str) -> i32{
    let game: Vec<&str>= line.split(":").collect();
    let split: Vec<&str>= game[1].split("|").collect();
    let winning = fill_set(&split[0]);
    let having = fill_set(&split[1]);

    let mut num_winning = 0;
    for n in having {
        if winning.contains(&n) {
            num_winning += 1; 
        }
    }

    return num_winning;
}

fn star1(lines :&Vec<&str>) {
        let base: i32 = 2;
        let mut star1 = 0;
        for line in lines {
            let num_winning = get_num_winning(&line);
            if num_winning>0 {
                star1 += base.pow((num_winning-1) as u32);
            }

        }
        println!("star1 {}", star1);
}
fn star2(lines :&Vec<&str>) {
        let mut star2 = 0;

        let mut multiply_card = vec![1; lines.len()];
        for i in 0..lines.len() {
            star2 += multiply_card[i];
            let line = &lines[i];
            let num_winning = get_num_winning(&line) as usize;
            for j in (i+1)..min(lines.len(), i+1+num_winning){
                multiply_card[j]+=multiply_card[i];
            }
        }
        println!("star2 {}", star2);
}

fn main() {

    let files = vec!["test", "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines: Vec<&str> = input.lines().collect();
        star1(&lines);
        star2(&lines);
       }
}
