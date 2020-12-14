extern crate lib;
use lib::io::*;
use std::cmp::{min,max};

fn main() {
    
   let line = read_all_from_stdin();
   // println!("{:?}",line);
     
   let input: Vec<i32> = line[..line.len()-1].split(&[' ', ',','\n'][..]).map(|x| {if ! x.eq("x") {x.parse::<i32>().unwrap()} else {-1}}).collect();

   // println!("{:?}", input);
    
    //star1
    let mut min_m = input[0];
    let mut min_idx = 0;

    for i in 1..input.len(){
        if input[i] == -1 { continue; }
        let modulo = input[0] % input[i];
     //   println!("{} {} {}", input[i], modulo,input[i] - modulo);
        if modulo == 0 {
            min_idx = i;
            min_m = 0;
            break;
        } 
        if input[i] - modulo < min_m {
            min_m = input[i] - modulo;
            min_idx = i;
        }
    }

   // println!("{} {} {}", input[min_idx], min_m, min_m + input[0]);
    println!("star1: {}", input[min_idx] * min_m);
}
