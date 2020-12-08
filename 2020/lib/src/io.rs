use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::process;

pub fn read_str_from_file(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    //println!("{}", &contents);
    Ok(contents)
}



