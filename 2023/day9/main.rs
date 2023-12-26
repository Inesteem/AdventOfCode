use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn read_inputs(filename: String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_num(s: &str) -> i64 {
    return s.parse::<i64>().unwrap();
}

fn is_zero(sequence: &Vec<i64>) -> bool {
    sequence.iter().all(|&item| item == 0)
}

fn predict_future(sequence: &Vec<i64>) -> i64 {
    if is_zero(sequence) {
        return 0;
    }

    let mut diffs = vec![0; sequence.len() - 1];
    for i in 1..sequence.len() {
        diffs[i - 1] = sequence[i] - sequence[i - 1];
    }

    sequence[sequence.len() - 1] + predict_future(&diffs)
}

fn restore_past(sequence: &Vec<i64>) -> i64 {
    if is_zero(sequence) {
        return 0;
    }

    let mut diffs = vec![0; sequence.len() - 1];
    for i in 1..sequence.len() {
        diffs[i - 1] = sequence[i] - sequence[i - 1];
    }

    sequence[0] - restore_past(&diffs)
}

fn main() {
    let files = vec!["test", "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) => input = inputs,
            Err(_) => process::exit(0),
        }
        let mut sum_fut: i64 = 0;
        let mut sum_pst: i64 = 0;
        let history: Vec<Vec<i64>> = input
            .lines()
            .map(|line| line.split(" ").map(|x| get_num(x)).collect())
            .collect();
        for sequence in history {
            sum_fut += predict_future(&sequence);
            sum_pst += restore_past(&sequence);
        }
        println!("star1: {}", sum_fut);
        println!("star2: {}", sum_pst);
    }
}
