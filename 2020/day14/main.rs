extern crate lib;
use lib::io::*;

use std::collections::HashMap;

struct Mask {
    ones : u64,
    zeroes : u64,
}


fn get_mask( bits : &str ) -> Mask {     
    let mask : Vec<char>= bits.chars().collect();
    let mut mask_one : u64 = 0;
    let mut mask_zero : u64 = !0;
    for i in 0..mask.len() {
        if mask[i] == '0' {
            mask_zero ^=  ((1 as u64) << (35-i)) as u64;
        }
        else if mask[i] == '1' {
            mask_one |=  ((1 as u64) << (35-i)) as u64;
        }
    }

    Mask{ones : mask_one, zeroes : mask_zero}
}



fn main() {
    let mut memory = HashMap::new();
    let mut mask = Mask{zeroes : 0, ones : 0}; 
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 {break;}
        let op : Vec<&str>= line.split_whitespace().collect();
        
        if op[0].eq("mask") {
            mask = get_mask(&op[2]);
            continue;
        }

        let mut val = op[2].parse::<u64>().unwrap();
        val |= mask.ones;
        val &= mask.zeroes;
        let mem = op[0][4..op[0].len()-1].parse::<usize>().unwrap();
        
        memory.insert(mem,val);
    }
    let mut ret : u64 = 0; 
    for mem in memory {
        println!("{:?}", mem);
        ret += mem.1;
    }
    println!("star1: {}", ret);
}


