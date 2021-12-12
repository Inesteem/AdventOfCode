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


fn frequencies(numbers : &Vec<u32>, bits : u32, ones : &mut Vec<u32>, zeroes : &mut Vec<u32>)
{
    for i in 0..numbers.len() {
        let mut num = numbers[i];
        for b in 0..bits {
            if (num & 0x1) == 0x1 {
                zeroes[(bits-b-1) as usize] += 1;
            } else {
                ones[(bits-b-1) as usize] += 1;
            }
            num >>= 1;
        }
    }

}


fn star1(numbers : &Vec<u32>, bits : u32) {

    //calculate frequencies
    let mut zeroes = vec![0 as u32; bits as usize];
    let mut ones = vec![0 as u32; bits as usize];
    frequencies(numbers, bits, &mut zeroes, &mut ones);

    let mut gamma : u32 = 0;
    for b in 0..bits {
        gamma <<= 1; 
        if zeroes[b as usize] > ones[b as usize] {
            gamma |= 0x1;
        }     
    }

    let epsilon = gamma ^ ((0x1 << bits) - 1);

    println!("{}", gamma * epsilon);

}

fn co2Cmp(a : usize, b :usize) -> bool {
    a <= b
}


fn oxyCmp(a : usize, b :usize) -> bool {
    a > b
}

fn getValue(bits : u32, numbers : &mut Vec<u32>, cmp : fn(a:usize,b:usize) -> bool) -> u32{

    let mut start = 0;
    let mut end = numbers.len();

    for b in (1..=bits).rev() {
        if end - start == 1 {
          break;  
        }

        let mask = (0x1 << b)-1;
        let bit = 0x1 << (b-1);

        numbers[start..end].sort_by(|a,b| (a & mask).cmp(&(b & mask)));

        let mid : usize = (end + start)/2;
        for n in start..end {
            if numbers[n] & bit != 0 {
                //less zeroes
                if cmp(n,mid)
                {
                    end = n;
                } else {
                    start = n;
                }
                break;
            }
        }
    }

    numbers[start]

}

fn main() {
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
    //let bits : u32 = 5;


    println!("\n\n {}", getValue(bits, &mut numbers, oxyCmp) * getValue(bits, &mut numbers, co2Cmp));
}
