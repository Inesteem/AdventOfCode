use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_idx(s : &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}


fn string_len(s : &str) -> usize {
    let v : Vec<char> = s.chars().collect();
    let mut num = 0;
    let mut i = 1;
    while i < s.len()-1 {
        if v[i] == '\\' { 
            if v[i+1] == 'x' { i += 3; }
            else {i += 1};
        }
        num += 1;
        i += 1;
    }
    num
}

fn string_len2(s : &str) -> usize {
    let mut num = 2;
    for c in s.chars() {
        num += match c {
            '"' | '\\'  => 2,
            _ => 1,
        };
    }
    num
}

fn main() {
    let files = vec!["test", "data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let lines : Vec<&str>= input.lines().collect();

        let mut memory = 0;
        let mut code = 0;
        let mut new_len = 0;
        for word in lines {
            code += word.len();
            memory += string_len(word);
            new_len += string_len2(word);

            //println!("{} {} {}", word.len(), string_len(word), string_len2(word));
        }
        println!("star1: {} - {} =  {}", code, memory, code - memory);
        println!("star2: {} - {} =  {}",new_len ,code, new_len - code);

        
   }

}
