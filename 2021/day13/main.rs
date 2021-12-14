use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::cmp::max;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}


fn cnt( field : &Vec<Vec<u8>>, Y : usize, X : usize) -> usize{

    let mut num = 0;
    for row in 0..Y {
        for col in 0..X {
            if field[row][col] == 0 {
                print!(".");
            } else {
                print!("#");
                num += 1;
            }
        }
        println!("");
    }

    num

}

fn main() {
    let width : usize = 1500;
    let height : usize= 1500;
    let files = vec!["test", "data"];

    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<&str> = input.lines().collect();

        let mut field : Vec<Vec<u8>> = vec![vec![0; width]; height];

        let mut pos = lines.len();
        let mut maxWidth = 0;
        let mut maxHeight = 0;
        //parse field
        for i in 0..lines.len() {
            let line = &lines[i];

            if line.len() == 0 {
                pos = i;
                break;
            }
            let coord : Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();

            maxWidth = max(maxWidth, coord[0]);
            maxHeight = max(maxHeight, coord[1]);
            field[coord[1]][coord[0]] = 1;
        }

        maxWidth += 1;
        maxHeight += 1;

        //parse folds
        for i in pos+1..lines.len() {
            let tmp : Vec<&str> = lines[i].split(" ").collect();
            let tmp2 : Vec<&str> = tmp[2].split("=").collect();

            let idx : usize = tmp2[1].parse().unwrap();

            if tmp2[0].chars().nth(0).unwrap() == 'y' {

                for row in idx+1..maxHeight {
                    let diff = row - idx;
                    for col in 0..maxWidth {
                        field[idx-diff][col] |= field[row][col];
                    }

                }

                maxHeight = idx;
            } else {

                for col in idx+1..maxWidth {
                    let diff = col - idx;
                    for row in 0..maxHeight {
                        field[row][idx-diff] |= field[row][col];
                    }

                }

                maxWidth = idx;
            }
        }
        
        println!("{}", cnt(&field, maxHeight, maxWidth));

    }
}
