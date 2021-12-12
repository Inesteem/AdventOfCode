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


#[derive(Debug,Clone)]
struct Position {
    board : usize, 
    row : usize,
    col : usize,
}

#[derive(Debug)]
struct Board {
    board : Vec<Vec<u32>>
}

trait BoardFuncs {
    fn fill_row(&mut self, row : &str, pos : usize, map : &mut Vec<Vec<Position>>);
    fn mark_and_check(&mut self, pos : &Position) -> bool;
    fn get_sum (&self) -> u32;
}


impl BoardFuncs for Board {

    fn fill_row(&mut self, row : &str, pos : usize, map : &mut Vec<Vec<Position>>){
        let numbers: Vec<u32> = row.split(" ")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let row : usize = self.board.len();
        for col in 0..numbers.len() {
            map[numbers[col] as usize].push(Position{board : pos, row : row, col : col});
        }

        self.board.push(numbers);

    }

    fn mark_and_check(&mut self, pos : &Position) -> bool {
        if self.board[pos.row][pos.col] == 100 { return false; }

        self.board[pos.row][pos.col] = 100;

        let mut full = true;
        //row
        for p in 0..5 {
            if self.board[pos.row][p] != 100{
                full = false;
                break;
            }
        }
        if full { return true; }

        full = true;
        //rcol
        for p in 0..5 {
            if self.board[p][pos.col] != 100{
                full = false;
                break;
            }
        }
        return full;
    }

    fn get_sum (&self) -> u32
    {
        let mut sum : u32 = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] != 100 {
                    sum += self.board[row][col];
                }
            }
        }
        sum
    }
}

impl Default for Board {
    fn default () -> Board {
        Board { board : vec![] }
    }

}

fn main() {
    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            input = inputs,
        Err(_) => process::exit(0),
    }
    let lines : Vec<&str> = input.lines().collect();

    let numbers: Vec<usize> = lines[0].split(",")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();

    println!("{:?}", numbers);

    let mut boards : Vec<Board> = vec![];
    let mut map = vec![vec![]; 100];

    for i in 1..lines.len() {
        if lines[i].len() == 0{
            boards.push(Board::default());
            continue;
        }
        let pos = boards.len()-1;
        boards[pos].fill_row(lines[i], pos, &mut map);
    }

    //star1
    //for n in numbers {
    //    for i in 0..map[n].len() {
    //        let pos = &map[n][i]; 
    //        if boards[pos.board].mark_and_check(&pos) {
    //            let sum = boards[pos.board].get_sum();
    //            println!("{}, {}, {} -> {}", n, pos.board, sum, sum * n as u32);
    //            process::exit(0);
    //        }
    //    }
    //}
    
    //star2
    let mut alreadyFin = vec![false; boards.len()];
    let mut numFin = 0;
    for n in numbers {
        for i in 0..map[n].len() {
            let pos = &map[n][i]; 
            if alreadyFin[pos.board] { continue; }
            if boards[pos.board].mark_and_check(&pos) {
                alreadyFin[pos.board] = true;
                numFin += 1;
                //last board marked:
                if numFin == boards.len(){
                    let sum = boards[pos.board].get_sum();
                    println!("{}, {}, {} -> {}", n, pos.board, sum, sum * n as u32);
                    process::exit(0);
                }
            }
        }
    }

}
