extern crate lib;
use lib::io::*;

use std::collections::HashMap;

#[derive(Debug)]
struct Mask {
    ones : u64,
    zeroes : u64,
    floating : u64,
}

impl Mask {

    fn change_val_v1(&self, val : u64) -> u64 {
        let mut v = val;
        v |= self.ones;
        v &= !self.zeroes;
        v 
    }

    fn rec_mem_update(&self, pos : u8, mem : u64, val : u64,  memory : &mut HashMap<u64,u64>) {
        let mut p = pos;
        while p < 36 && (self.floating & (1u64 << p) == 0){
            p += 1;
        }
        if p == 36 {
            memory.insert(mem,val);
            return;
        }

        self.rec_mem_update(p+1, mem,val,memory);
        let mut m = mem;
        m |=  1u64 << p;
        self.rec_mem_update(p+1, m,val,memory);
        
    }

    fn update_memory(&self, mem : u64, val : u64,  memory : &mut HashMap<u64,u64>) {
        let mut m = mem;
        m |= self.ones;
        m &= !self.floating;
        self.rec_mem_update(0,m,val,memory);
    }
}

fn get_mask( bits : &str ) -> Mask {     
    let mask : Vec<char>= bits.chars().collect();
    let mut mask_one : u64 = 0;
    let mut mask_zero : u64 = 0;
    for i in 0..mask.len() {
        if mask[i] == '0' {
            mask_zero ^=  (1u64 << (35-i)) as u64;
        }
        else if mask[i] == '1' {
            mask_one |=  (1u64 << (35-i)) as u64;
        }
    }

    Mask{ones : mask_one, zeroes : mask_zero, floating : !(mask_one | mask_zero)}
}



fn main() {

    let star1 = false;
    let mut memory = HashMap::new();
    let mut mask = Mask{zeroes : 0, ones : 0, floating : 0}; 
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 {break;}
        let op : Vec<&str>= line.split_whitespace().collect();
        
        if op[0].eq("mask") {
            mask = get_mask(&op[2]);
 //           println!("{}:\n{:#066b}\n{:#066b}\n{:#066b}\n", op[2],mask.ones, mask.zeroes, mask.floating);
            continue;
        }

        let mut val = op[2].parse::<u64>().unwrap();
        let mem = op[0][4..op[0].len()-1].parse::<u64>().unwrap();
        //star1
        if star1 {
            val = mask.change_val_v1(val);
            memory.insert(mem,val);
        } else {
            mask.update_memory(mem,val,&mut memory);
        }

    }
    let mut ret : u64 = 0; 
    for mem in memory {
        ret += mem.1;
    }
    if star1 {
        println!("star1: {}", ret);
    } else {
        println!("star2: {}", ret);
    }
}


