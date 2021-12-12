use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::cmp::max;
use std::cmp::min;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}


#[derive(Debug,Clone)]
struct Position {
    row : usize,
    col : usize,
}


impl Default for Position {
    fn default () -> Position {
        Position { row : 0, col : 0 }
    }
}
trait Parse {
    fn parse (s : &str) -> Self;
} 

impl Parse for Position {
    fn parse (s : &str) -> Position {
        let numbers : Vec<usize> = s.split(",")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
        Position { row : numbers[0], col : numbers[1] }
    }
}

#[derive(Debug,Clone)]
struct Line {
    a: Position,
    b: Position,
}

impl Parse for Line {
    fn parse (s : &str) -> Line {
        let mut p: Vec<Position> =s.split(" -> ")
            .map(|x| Position::parse(x)) 
            .collect();
            if p[0].col < p[1].col {
                Line {a : p.remove(0), b : p.remove(0)}
            } else {
                Line {a : p.remove(1), b : p.remove(0)}
            }
    }

}
trait Stuff {
    fn is_vertical_line(&self) -> bool;
    fn is_horizontal_line(&self) -> bool;
}

impl Stuff for Line
{
    fn is_vertical_line(&self) -> bool {
        self.a.col == self.b.col
    }

    fn is_horizontal_line(&self) -> bool {
        self.a.row == self.b.row
    }

}



fn main() {
    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            input = inputs,
        Err(_) => process::exit(0),
    }
    let lines : Vec<&str> = input.lines().collect();

    let mut board : Vec<Vec<u32>> = vec![vec![0;1000];1000];

    for line in lines {
        let l = Line::parse(line);

        //star1
        if l.is_horizontal_line() {
            for i in l.a.col..=l.b.col
            {
                board[l.a.row][i] += 1;
            }
        }
        else if l.is_vertical_line() {
            for i in min(l.a.row, l.b.row)..=max(l.a.row, l.b.row)
            {
                board[i][l.a.col] += 1;
            }
        } else {
            let mut step : i32 = 1;
            if l.b.row < l.a.row {
                step = -1; 
            }

            let mut row : i32 = l.a.row as i32;
            for col in l.a.col..=l.b.col
            {
                board[row as usize][col] += 1;
                row += step;
            }
        }
    
    }

    let mut num : u32 = 0;
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] > 1 {
                num += 1
            }
        }
    }

    println!("{}", num);
}
