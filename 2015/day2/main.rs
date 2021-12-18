use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn read_in_one_char() {
    let input: Option<i32> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
}
//lwh
fn needed_materials(dims : &mut Vec<usize>) -> (usize, usize) {
    let mut ribbon = 0;
    dims.sort();
    ribbon = 2*dims[0] + 2*dims[1] + dims[0]*dims[1]*dims[2];

    let l = dims[0];
    let w = dims[1];
    let h = dims[2];
    let mut calced : Vec<usize> = vec![l*w, w*h, h*l];
    calced.sort();
    let paper = 2*calced[0] + 2*calced[1] + 2*calced[2] + calced[0];
    (paper, ribbon)
}

fn main() {
    let files = vec!["test", "data"];
    let star1 = false;
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }
        let lines : Vec<&str> = input.lines().collect();
        
        let mut paper = 0;
        let mut ribbon = 0;
        for line in lines {
            let mut dims : Vec<usize> = line.split("x").map(|x| x.parse().unwrap()).collect();
            let ret = needed_materials(&mut dims);
            paper += ret.0;
            ribbon += ret.1;
        }
        println!("star1: {}", paper);
        println!("star2: {}", ribbon);
    }
}
