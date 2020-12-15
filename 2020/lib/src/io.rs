use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read};
//use std::process;

pub fn read_line_from_stdin() -> std::io::Result<String> {
    let mut buffer = String::new();

    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}


pub fn read_all_from_stdin() -> String {
    let mut ret: String = read_line_from_stdin().unwrap();
    
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break; }
        ret.push_str(&line);
    }
    ret
}

pub fn read_str_from_file(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    //println!("{}", &contents);
    Ok(contents)
}


pub fn read_in_int() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    return -1
}

