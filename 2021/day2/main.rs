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
    let slidingWindowSize = 3;
    let mut input: String;
    match read_inputs("input".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            input = inputs,
        Err(_) => process::exit(0),
    }
    let lines : Vec<&str> = input.lines().collect();

    let mut depth : i64 = 0;
    let mut pos : i64 = 0;
    let mut aim : i64 = 0;
    for i in 0..lines.len() {
        let cmds : Vec<&str> = lines[i].split_whitespace().collect();

        let steps : i64 = cmds[1].parse().expect("Not an integer!");

        match cmds[0] {
            "forward" => {
                pos += steps;
                depth += aim * steps;
            },
            "up" => aim -= steps,
            "down" => aim += steps,
            _ => process::exit(0),
        }
    }

    println!("{}",depth * pos);

}
