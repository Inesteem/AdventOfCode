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

fn print( board : &Vec<Vec<u8>>) {

    for row in board {
        for o in row {
            print!("{} ", o);
        }
        println!("");
    }

}

fn fire(board : &mut Vec<Vec<u8>>, row : isize, col : isize, fired : &mut Vec<Vec<bool>>) -> u64{
    fired[row as usize][col as usize] = true;
    board[row as usize][col as usize] = 0;

    let mut f = 1;

    for r in row-1..=row+1 {
        if r < 0 || r as usize >= board.len() { continue; }

        for c in col-1..=col+1 {
            if c < 0 || c as usize >= board[row as usize].len() {continue; }
            if fired[r as usize][c as usize] { continue; }

            board[r as usize][c as usize] += 1;
            if board[r as usize][c as usize] > 9 {
                f += fire(board, r, c, fired);
            }
        }
    }

    return f;
}

fn main() {

    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>
            input = inputs,
        Err(_) => process::exit(0),
    }

    let lines : Vec<&str> = input.lines().collect();

    let mut board : Vec<Vec<u8>> = vec![];
    let mut fired : Vec<Vec<bool>> = vec![vec![false;lines[0].len()]; lines.len()];

    for line in lines {
        board.push(line.chars().map(|c|  c.to_digit(10).unwrap() as u8).collect());
    }


    //let mut fireNum : u64 = 0;
    //for i in 0..100 {
    let numOcti = board.len() * board[0].len();
    let mut step : u64= 0;
    loop {
        step += 1;
        for row in 0..board.len() {
             for col in 0..board[row].len() {
                if fired[row as usize][col as usize]
                {
                    continue;
                }

                board[row][col] += 1;
                if board[row][col] > 9 {
                    //fireNum += fire(&mut board, row as isize, col as isize, &mut fired);
                    let fireNum = fire(&mut board, row as isize, col as isize, &mut fired);
                    if fireNum as usize == numOcti {
                        println!("first time all flash: {}", step);
                        process::exit(0);

                    }
                }
             }
        }

        for row in 0..fired.len() {
             for col in 0..fired[row].len() {
                 fired[row as usize][col as usize] = false;
             }
        }
        //println!("\n{}: {}\n", i, fireNum);
    }
}
