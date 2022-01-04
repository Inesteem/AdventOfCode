use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::isize;

const DEBUG : bool = false;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_light(board : &[Vec<bool>], row : isize, col : isize) -> bool {

    let mut lit = 0;
    for r in row-1..=row+1 {
        for c in col-1..=col+1 {
            if r < 0 ||
               c < 0 ||
               r >= (board.len() as isize) ||
               c >= (board[0].len() as isize) {
                   continue;
               }

            if board[r as usize][c as usize]{
                lit += 1;
            }
        }
    }

    // the light it self is counted as well
    if board[row as usize][col as usize] { return lit == 3 || lit == 4;}
    return lit == 3;

}
fn lit_all_corners(board : &mut Vec<Vec<bool>>) {
    //if star1: disable
    let width = board[0].len();
    let height= board.len();
    board[0][0] = true;
    board[0][width-1] = true;
    board[height-1][0] = true;
    board[height-1][width-1] = true;
}
fn main() {
//    std::io::stdin().read_to_string(&mut input).unwrap();
    let files = vec!["data"];
//    let files = vec!["test", "data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let lines : Vec<Vec<bool>>= input.lines()
            .map(|x| x.chars().map(|y|
                                   match y {
                                       '#' => true,
                                       _ => false,
                                   }
                                   ).collect())

            .collect();

        let mut new_board;
        let mut board : Vec<Vec<bool>> = lines.iter().cloned().collect();
        lit_all_corners(&mut board);

        let width = board[0].len();
        let height= board.len();

        for i in 1..=100 {

            new_board = vec![vec![false; width]; height];

            for row in 0..height {
                for col in 0..width {

                    new_board[row][col] = get_light(&board, row as isize, col as isize);

                    if new_board[row][col] {
                        if DEBUG {print!("#");}
                    } else {
                        if DEBUG {print!(".");}
                    }
                }

                if DEBUG {println!();}
            }

            board = new_board;
            lit_all_corners(&mut board);

            let mut lit = 0;
            for row in &board { for light in row { if *light { lit += 1; } } }
            if true {println!("\n -> {}: {}\n\n", i, lit);}
        }
    }
}
