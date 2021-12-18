use std::fs::File;
use std::io::BufReader;
use std::process;
use std::any::TypeId;
use priority_queue::PriorityQueue;
use std::io::Read;
use num::NumCast;
use termion::color;

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

fn print<T: std::fmt::Debug + std::fmt::Display + 'static + std::ops::Add<Output = u8 > + num::ToPrimitive + NumCast + Copy>( board : &Vec<Vec<T>>, cmp : &Vec<Vec<u8>>) {

    let width = cmp[0].len();
    let height= cmp.len();
    for row in 0..height*5{
        for col in 0..width*5 {

            let increase = (col / width) + (row / height);
            let risk : usize = num::cast(board[row%height][col%width]).unwrap();
            let val = ( (risk + increase ) % 10 + ((risk + increase )/ 10) as usize);
            if TypeId::of::<T>() == TypeId::of::<bool>() {
                if (board[row][col]).to_string() == "true" {
                    print!("{}", cmp[row][col]);
                } else {
                    print!(".");
                }
            } else {

                //if (col / width) as usize == (row / height) as usize {
    //                print!("{}{:?}{}", color::Fg(color::Red), ( (risk + increase ) % 10) as u8, color::Fg(color::White));
                //} else {
                    print!("{:?}", val);
                //}
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

    let width = board[0].len() as isize;
    let height= board.len() as isize;
    let mut visited : Vec<Vec<bool>> = vec![vec![false; 5*width as usize]; 5*height as usize];
    let endCoord = Coord { row : (5 * height as usize)-1, col : (5 * width as usize)-1, path : 0};

    let mut todo = PriorityQueue::new();

    todo.push( Coord { row : 0, col : 0, path : 0}, 0 as isize);

    while !todo.is_empty() {
        let (coord , path) = todo.pop().unwrap();
        //read_char();
        //print(&visited, &board);println!("");
        //        visited[coord.row as usize][coord.col as usize] = true;

        for (r,c) in &[(1,0), (0,1), (-1,0),(0,-1)] {
            let mut row = coord.row as isize + r;
            if row < 0 || row >= 5*height { continue; }

            let mut col = coord.col as isize + c;
            if col < 0 || col >= 5*width { continue; }

            let increase = (col / width) as usize + (row / height) as usize;

            if visited[row as usize][col as usize] { continue; }
            visited[row as usize][col as usize] = true;


            let mut path = board[(row % height) as usize][(col % width) as usize] as usize + increase;
            path = (path % 10) + (path / 10) + coord.path;
            let nextCoord = Coord{ row : row as usize, col : col as usize, path : path};
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
