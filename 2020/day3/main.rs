use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::io;
use std::ops::AddAssign;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
/*
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}


impl<T:Add> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Point<T>){
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}*/



fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    //println!("{}", &contents);
    Ok(contents)
}

fn main() {
    let map: Vec<Vec<char> >;
    match read_inputs("dat".to_string()) {
        Ok(i) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            map = i.split('\n')
            .map(|x| x.to_string().chars().collect())
            .collect()
            ,
        Err(_) => process::exit(0),
    }
    println!("{:?}", &map);

    let slope = Point{x:3,y:1};
    let mut pos = Point{x:0,y:0};
    let mut trees =0;
    while pos.y < map.len() {
        if map[pos.y][pos.x] == '#' {
            trees += 1;
        }
        println!("{:?} -> {}", pos,trees);
        pos.add_assign(slope);
        pos.x %= map[0].len();
    }
}
