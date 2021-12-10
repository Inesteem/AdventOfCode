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
    let mut numbers: Vec<u32>;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            numbers = inputs.split("\n")
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect(),
        Err(_) => process::exit(0),
    }

    let bits : u32 = 12;
    let mut zeroes = vec![0; bits as usize];
    let mut ones = vec![0; bits as usize];

    for i in 0..numbers.len() {
        let mut num = numbers[i];
        for b in 0..bits {
            if (num & 0x1 == 0x1) {
                zeroes[(bits-b-1) as usize] += 1;
            } else {
                ones[(bits-b-1) as usize] += 1;
            }
            num >>= 1;
        }
    }
    
    let mut gamma : u32 = 0;
    for b in 0..bits {
        gamma <<= 1; 
        if zeroes[b as usize] > ones[b as usize] {
            gamma |= 0x1;
        }     
    }

    let epsilon = gamma ^ ((0x1 << bits) - 1);

//   println!("{} {}", gamma, epsilon);
   println!("{}", gamma *  epsilon);
    
}
