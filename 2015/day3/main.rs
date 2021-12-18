use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::collections::HashSet;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

#[derive (Debug, Hash, Eq, Clone, Copy)]
struct Coord {
    x : isize,
    y : isize,

}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x== other.x && self.y == other.y
    }
}

fn read_in_one_char() {
    let input: Option<i32> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
}

struct Santa {
    pos : Coord,
}

impl Santa {
    pub fn new() -> Self {
        Santa {pos : Coord{ x : 0, y : 0}}
    }

    pub fn moveSanta(&mut self, dir : char) {
        match dir {
            '^' => { self.pos.y += 1;}
            'v' => { self.pos.y -= 1;}
            '>' => { self.pos.x += 1;}
            '<' => { self.pos.x -= 1;}
            _ => {process::exit(0);}
        }
    }
}

fn main() {
    let star1 = false;
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
        for line in lines {
            let mut set = HashSet::new();

            let mut santa = Santa::new();
            let mut robo = Santa::new();
            set.insert(santa.pos);
            let v : Vec<char>= line.chars().collect();

            let mut step = 2;
            if star1 { step = 1;}

            for i in (0..line.len()-1).step_by(2) {
                santa.moveSanta(v[i]);
                set.insert(santa.pos);
                robo.moveSanta(v[i+1]);
                set.insert(robo.pos);
            }
            println!("{}", set.len());
        }
    }
}
