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

pub fn read_str_from_file(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    //println!("{}", &contents);
    Ok(contents)
}




