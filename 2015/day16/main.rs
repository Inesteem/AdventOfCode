use std::fs::File;
use std::io::BufReader;
use std::process;
use std::io::prelude::*;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

#[derive(Debug, Clone)]
struct AuntAttributes {

    children: isize,
    cats: isize,
    samoyeds: isize,
    pomeranians: isize,
    akitas: isize,
    vizslas: isize,
    goldfish: isize,
    trees: isize,
    cars: isize,
    perfumes: isize,
}
impl AuntAttributes {

    fn new() -> Self {
        AuntAttributes{ children : -1, cats : -1, samoyeds : -1, pomeranians : -1, akitas : -1, vizslas : -1, goldfish : -1, trees : -1, cars : -1, perfumes : -1 }
    }

}
fn main() {
    /*
    children: 3
    cats: 7
    samoyeds: 2
    pomeranians: 3
    akitas: 0
    vizslas: 0
    goldfish: 5
    trees: 3
    cars: 2
    perfumes: 1
    */
    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>
            input = inputs,
        Err(_) => process::exit(0),
    }

    let lines : Vec<Vec<String>>= input.lines().map(|line| line.split_whitespace().
                                                  map(|word| word.replace(&[',', ':',][..], "")).

                                                  collect()).collect();
    'outer: for line in &lines {
        let idx : usize = line[1].parse().unwrap();
        
        for i in (2..line.len()).step_by(2) {
            let value : usize = line[i+1].parse().unwrap();
            match &line[i][..] {
                //set all <= / >= to != for star1
                "children" => if value != 3 { continue 'outer; },
                "cats" => if value <= 7 { continue 'outer; },
                "samoyeds" => if value != 2 { continue 'outer; },
                "pomeranians" => if value >= 3 { continue 'outer; },
                "akitas" => if value != 0 { continue 'outer; },
                "vizslas" => if value != 0 { continue 'outer; },
                "goldfish" => if value >= 5 { continue 'outer; },
                "trees" => if value <= 3 { continue 'outer; },
                "cars" => if value != 2 { continue 'outer; },
                "perfumes" => if value != 1 { continue 'outer; },
                _ => panic!("strange input {}", line[i]),
            }
        }
        println!("star2: aunt {} is a match", idx);
    }
}
