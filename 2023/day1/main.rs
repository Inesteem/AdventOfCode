use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use regex::Regex;
use std::collections::HashMap;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_first_digit(line: Vec<char>, re: &regex::Regex, m: &HashMap<&str, u32>) -> u32 {
    let s: String = line.into_iter().collect();
    let Some(caps) = re.captures(&s) else {
        return 0;
    };
    return m[&caps["digit"]];
}

fn main() {
    let digit_map : HashMap<&str, u32> = [
        ("0", 0),
        ("zero", 0),
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
        ].iter().cloned().collect();

    let digit_map_rev : HashMap<&str, u32> = [
        ("0", 0),
        ("orez", 0),
        ("1", 1),
        ("eno", 1),
        ("2", 2),
        ("owt", 2),
        ("3", 3),
        ("eerht", 3),
        ("4", 4),
        ("ruof", 4),
        ("5", 5),
        ("evif", 5),
        ("6", 6),
        ("xis", 6),
        ("7", 7),
        ("neves", 7),
        ("8", 8),
        ("thgie", 8),
        ("9", 9),
        ("enin", 9),
        ].iter().cloned().collect();

    let re = Regex::new(r"(?<digit>one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let reverse_re = Regex::new(r"(?<digit>enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|[0-9])").unwrap();

    let files = vec!["test", "test2", "input"];
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
            sum += get_first_digit(line.chars().collect(), &re, &digit_map) * 10;
            sum += get_first_digit(line.chars().rev().collect(), &reverse_re, &digit_map_rev);
        }
        println!("{}", sum);
    }
}
