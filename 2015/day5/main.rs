use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}



fn is_nice( word : &str) -> bool {
    let mut numVocals = 0;
    let mut doubles = false;
    let mut lastC = "";

    for i in 0..word.len() {
        let c = &word[i..i+1];
        for vocal in &["a","e","o","u","i"] {
            if &c == vocal {
                numVocals += 1;
            }
        }
        if c == lastC { 
            doubles = true;
        }
        lastC = c;
    }

    return doubles && numVocals > 2;
}

fn is_naughty( word : &str) -> bool {
    
    for i in 0..word.len()-1 {
        for substr in &["ab", "cd", "pq","xy"] {
            if &word[i..i+2] == *substr { return true;}
        }
    }

    return false;
}

fn main() {
    let files = vec!["test", "data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<&str>= input.lines().collect();

        let mut niceOnes = 0;
        for line in lines {
            if is_nice(&line) && !is_naughty(&line) {
                niceOnes += 1;
            } 
        
        }
        println!("star1: {}", niceOnes );
    }
}
