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




fn main() {
    let mut numbers: Vec<u32>;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            numbers = inputs.split("\n")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect(),
        Err(_) => process::exit(0),
    }
    let mut increased : u32 = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i-1] {
            increased += 1;
        }
    }
    println!("{}", increased);
}
