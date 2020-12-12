use std::process::exit;
use std::cmp::{min,max};
extern crate lib;
use lib::io::*;


fn main() {
    let mut numbers = vec![];
    loop {
       let line = read_line_from_stdin().unwrap(); 
       if line.len() == 0 { break; }

       numbers.push(line[0..line.len()-1].parse::<u64>().unwrap());
    }
    println!("{:?}", numbers);

    //parsing done
    let preamble_size = 25;
    let mut weakness = 0;
    for i in preamble_size..numbers.len() {
        
        let mut found = false;
        for j in i-preamble_size..i {
            
            for k in i-preamble_size..i {
                if k == j { continue; }

                if numbers[k] + numbers[j] == numbers[i] {
                    found = true;
                    break;
                }
            }
        }
        if !found { 
            println!("star1: {}",numbers[i]);
            weakness = i;
            break;
        } 
    }

    for i in 0..weakness {
        let mut num = numbers[i];
        let mut j = i;
        while num < numbers[weakness] && j < weakness {
            j += 1;
            num += numbers[j];
        }
        if num==numbers[weakness] {
            let mut smallest = numbers[j];
            let mut largest = numbers[j];
            for k in i..j {
               smallest = min(smallest,numbers[k]); 
               largest = max(largest,numbers[k]); 
            }

            println!("star2: {} + {} = {}", smallest, largest, smallest + largest);
            break;
        }
    }

}
