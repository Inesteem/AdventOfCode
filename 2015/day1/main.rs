use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::cmp::max;

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

fn main() {
    let files = vec!["data", "test", "test2"];
    let star1 = false;
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }
        let mut floor : isize = 0;
        for c in input.chars() {
            if c == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }
        }
        println!("star1: {}", floor);

        let mut pos = 0;
        floor= 0;
        for c in input.chars() {
            if floor == -1 { break; }
            if c == '(' {
                floor += 1;
            } else {
                floor -= 1;
            }
            pos += 1;
        }

        println!("star2: {}", pos);
    }
}
