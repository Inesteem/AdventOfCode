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

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input_vec.clone();
            program[1] = noun;
            program[2] = verb;
        //    program = [1,0,0,0,99].to_vec();

            let mut i : usize = 0;
            while i < program.len() {
                
                if  program[i] == 99  {
                    break;
                }

                let left_op = program[program[i+1] as usize];
                let right_op = program[program[i+2] as usize];
                let idx_to = program[i+3] as usize;
                if program[i] == 1 {
                    program[idx_to] = left_op + right_op;
                } else if program[i] == 2 {
                    program[idx_to] = left_op * right_op;
                } else {
                    assert!(false);
                } 

                i += 4;
            }
            if program[0] == 19690720 {
                println!("{:?}", program);
                println!("end found: {} {} -> {}", noun, verb, noun*100+verb);

            }
        }
    }
}
