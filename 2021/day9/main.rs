use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::cmp::min;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn star1(field: Vec<Vec<char>>) {
    println!("{:?}", field);

    let mut risk = 0;

    for r in 0..field.len() {
        
        for c in 0..field[r].len() {
            let curr = field[r][c];
            if r != 0 && field[r-1][c] <= curr {
                continue;
            }
            if r != field.len()-1 && field[r+1][c] <= curr {
                continue;
            }
            if c != 0 && field[r][c-1] <= curr {
                continue;
            }
            if c != field[r].len()-1 && field[r][c+1] <= curr {
                continue;
            }

            risk += curr.to_digit(10).unwrap() + 1;
        }
    }

    println!("risk: {}", risk);
}

fn fill(field: &mut Vec<Vec<char>>, last : i32, row : usize, col : usize) -> u32{
    if row < 0 || row >= field.len() { return 0; }
    if col < 0 || col >= field[row].len() { return 0; }
    if field[row][col] == '*' || field[row][col] == '9'{ return 0; }

    let curr : i32= (field[row][col].to_digit(10).unwrap()) as i32;
    if curr <= last { return 0; }
    field[row][col] = '*';
    let mut sum = 1;
    sum += fill(field, curr, row+1, col);
    sum += fill(field, curr, row-1, col);
    sum += fill(field, curr, row, col+1);
    sum += fill(field, curr, row, col-1);
    
    sum
}

fn star2(field: &mut Vec<Vec<char>>) {
    println!("{:?}", field);

    let mut risk = 0;

    let mut basins : Vec<u32>= vec![];

    for r in 0..field.len() {
        
        for c in 0..field[r].len() {
            let curr = field[r][c];
            if curr == '*' || curr == '9' {
                continue;
            }
            if r != 0 && field[r-1][c] <= curr {
                continue;
            }
            if r != field.len()-1 && field[r+1][c] <= curr {
                continue;
            }
            if c != 0 && field[r][c-1] <= curr {
                continue;
            }
            if c != field[r].len()-1 && field[r][c+1] <= curr {
                continue;
            }

            let sum = fill(field, -1, r, c);
            println!("{}", sum);
            basins.push(sum);
        }
    }
    
    basins.sort();
    basins.sort_by(|a, b| b.cmp(a));
    println!("risk: {}", basins[0]*basins[1]*basins[2]);

}

fn main() {

    let mut input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            input = inputs,
        Err(_) => process::exit(0),
    }

    let lines : Vec<&str> = input.lines().collect();

    let mut field : Vec<Vec<char>>  = vec![];

    for line in lines {
        field.push(line.chars().collect());
    }

    star2(&mut field);

}
