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
//fn read_in_one_char() {
//    let input: Option<i32> = std::io::stdin()
//        .bytes()
//        .next()
//        .and_then(|result| result.ok())
//        .map(|byte| byte as i32);
//}

fn get_index(board : &[Vec<bool>], row : isize, col : isize, infinity_val : bool) -> usize {
    
    let mut result = 0;
    for r in row-1..=row+1 {
        for c in col-1..=col+1 {
            result <<= 1;

            if r < 0 || 
               c < 0 || 
               r >= (board.len() as isize) || 
               c >= (board[0].len() as isize) { 
                    if infinity_val { result |= 1; } 
                   continue; }

            if board[r as usize][c as usize]{
                result |= 1; 
            }
        }
    }

    result
}

fn main() {
//    std::io::stdin().read_to_string(&mut input).unwrap();
    let files = vec!["test"];
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

        let encoding = &lines[0];

        let mut new_board;
        let mut board : Vec<Vec<bool>> = lines[2..lines.len()].iter().cloned().collect();

        for i in 0..3 {

            let width = board[0].len();
            let height= board.len();
            new_board = vec![vec![false; width+2]; height+2];
            let mut lit = 0;
            let mut dark = 0;
            println!("{} {}",width, height);
            let mut infinity_val = false;
            if i % 2 == 0 {
                infinity_val = encoding[0];
            }

            for row in -1..=height as isize {
                for col in -1..=width as isize {
                
                    let idx = get_index(&board, row, col, infinity_val);
                    new_board[(row+1) as usize][(col+1) as usize] = encoding[idx];

                    if encoding[idx] {
                        print!("#");
                        lit += 1;
                    } else {
                        print!(".");
                        dark += 1;
                    }
                }

                println!();
            }

            board = new_board;
            println!("\n{} pixels are lit!\n", lit);
        }
    }
}
