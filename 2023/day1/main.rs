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

fn get_first_digit(line : Vec<char>) -> u32 {
    for c in line {
        if c.is_digit(10) {
            return c.to_digit( 10).unwrap();
        }
    }
    return 0;
}

fn main() {

    let files = vec!["test", "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let mut sum : u32 = 0;
        let lines : Vec<&str> = input.lines().collect();
        for line in lines {
            let code = get_first_digit(line.chars().collect()) * 10 + get_first_digit(line.chars().rev().collect());
            sum += code;
        }
        println!("{}", sum);
    }
}
