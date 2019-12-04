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
    println!("{}", &contents);
    Ok(contents)
}

fn main() {
    let mut input_vec : Vec<u32>;
    match read_inputs("../../data/day2.txt".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            input_vec = inputs.split(",")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect(),
        Err(_) => process::exit(0),
            
    }
    input_vec[1] = 12;
    input_vec[2] = 2;
//    input_vec = [1,0,0,0,99].to_vec();

    let mut i : usize = 0;
    while i < input_vec.len() {
        
        if  input_vec[i] == 99  {
            break;
        }

        let left_op = input_vec[input_vec[i+1] as usize];
        let right_op = input_vec[input_vec[i+2] as usize];
        let idx_to = input_vec[i+3] as usize;
        if input_vec[i] == 1 {
            input_vec[idx_to] = left_op + right_op;
        } else if input_vec[i] == 2 {
            input_vec[idx_to] = left_op * right_op;
        } else {
            assert!(false);
        } 

        i += 4;
    }
    println!("{:?}", input_vec);
}
