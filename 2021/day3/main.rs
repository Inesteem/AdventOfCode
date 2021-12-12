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

//sort 1 at the start and 0 at the end
fn order(numbers : &mut [u32], bit : u32) ->  usize {
    let mask = 0x1 << bit;

    let mut start = 0;
    let mut end = numbers.len() - 1;

    loop {
        while start != end {
            if (numbers[start] & mask) == 0 {
                break;
            }
            start += 1;
        }
        while start != end {
            if (numbers[end] & mask) != 0 {
                break;
            }
            end -= 1;
        }

        if start == end {
            break;
        }

        let tmp = numbers[start];
        numbers[start] = numbers[end];
        numbers[end] = tmp;
    }

    start
}
// assumption: each col hast at least 1 zero and 1 one
// (as proven by known input)
fn oxy(bits : u32, numbers : &mut Vec<u32>) -> u32{


    let mut start : usize= 0;
    let mut end : usize = numbers.len();
    let mut mid : usize = 0;

    for b in (0..bits).rev() {
        println!("{} {} {}", start, end, mid);
        if(start - end <= 1) {
            return numbers[start];
        }
        mid = order(&mut numbers[start..end], b);
        // more zeroes
        if mid < ((end - start) / 2) as usize {
            start = mid + 1;
            println!("0");
        }
        else 
        {
            end = mid + 1;
            println!("1");
        }

        for i in start..end {
            println!("{:05b}", numbers[i]);
        }


    }
    println!("");
    numbers[mid]

}

fn main() {
    let slidingWindowSize = 3;
    let mut numbers: Vec<u32>;
    match read_inputs("test".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            numbers = inputs.split("\n")
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect(),
        Err(_) => process::exit(0),
    }

    //let bits : u32 = 12;
    let bits : u32 = 5;



    println!("\n\n {}", oxy(bits, &mut numbers));
}
