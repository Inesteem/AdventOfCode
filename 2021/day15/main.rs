use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::any::TypeId;
use priority_queue::PriorityQueue;
use std::io::Read;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn read_char() ->  i32{
    let input: Option<i32> = std::io::stdin()
    .bytes() 
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as i32);
    input.unwrap()
}

fn print<T: std::fmt::Debug + std::fmt::Display + 'static>( board : &Vec<Vec<T>>, cmp : &Vec<Vec<u8>>) {

    for row in 0..board.len(){
        for col in 0..board[row].len() {
            if TypeId::of::<T>() == TypeId::of::<bool>() {
                if (board[row][col]).to_string() == "true" {
                    print!("{}", cmp[row][col]);
                } else {
                    print!(".");
                }
            } else {
                print!("{:?}", board[row][col]);
            }

        }
        println!("");
    }

}


#[derive(Debug,Hash)]
struct Coord {
    row : usize,
    col : usize,
    path : usize,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }

}

trait ManhattenDistance {

    fn manhatten_distance(&self, c : &Coord) -> usize;
}

impl ManhattenDistance for Coord {

    fn manhatten_distance(&self, c : &Coord) -> usize {
        (num::abs(self.row as isize - c.row as isize) + num::abs(self.col as isize - c.col as isize)) as usize

    
    }
}
impl Eq for Coord {}

fn main() {

    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>
            input = inputs,
        Err(_) => process::exit(0),
    }

    let lines : Vec<&str> = input.lines().collect();

    let mut board : Vec<Vec<u8>> = vec![];

    for line in lines {
        board.push(line.chars().map(|c|  c.to_digit(10).unwrap() as u8).collect());
    }

    let mut visited : Vec<Vec<bool>> = vec![vec![false;board[0].len()];board.len()];
    let endCoord = Coord { row : board.len()-1, col : board[0].len()-1, path : 0};

    let mut todo = PriorityQueue::new();

    todo.push( Coord { row : 0, col : 0, path : 0}, 0 as isize);

    while !todo.is_empty() {
        let (coord , path) = todo.pop().unwrap();
        //read_char();
        //print(&visited, &board);println!("");
//        visited[coord.row as usize][coord.col as usize] = true;

        for (r,c) in &[(1,0), (0,1), (-1,0),(0,-1)] {
            let row = coord.row as isize + r;
            if row < 0 || row as usize >= board.len() { continue; }
                let col = coord.col as isize + c;
                if col < 0 || col as usize >= board[row as usize].len() { continue; }

                if visited[row as usize][col as usize] { continue; }
                visited[row as usize][col as usize] = true;


                let nextCoord = Coord{ row : row as usize, col : col as usize, path : coord.path + board[row as usize][col as usize] as usize};
                if nextCoord == endCoord {
                    println!("{}", nextCoord.path );
                    process::exit(0);
                }
                let priority = -(nextCoord.path as isize);
                //let priority = -(nextCoord.path as isize + (endCoord.manhatten_distance(&nextCoord) as isize)) as isize;
                todo.push( nextCoord, priority);
        }
    }
}
