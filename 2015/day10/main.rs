use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;





fn main() {

    let mut numbers : Vec<u8> = vec![1,3,2,1,1,3,1,1,1,2];

    for i in 0..50 {
        let mut last = numbers[0];
        let mut next = vec![];
        let mut num = 1;
        for n in &numbers[1..numbers.len()] {
            if *n == last {
                num += 1;
            } else {
                next.push(num);
                next.push(last);
                last = *n;
                num = 1;
            }

        }
        next.push(num);
        next.push(last);

        numbers = next;
    }
    println!("{}", numbers.len());


}
