extern crate lib;
use lib::io::*;
use std::cmp::{min,max};

fn main() {
    
   let line = read_all_from_stdin();
     
   let input: Vec<i64> = line[..line.len()-1].split(&[' ', ',','\n'][..]).map(|x| {if ! x.eq("x") && x.len()>0 {x.parse::<i64>().unwrap()} else {-1}}).collect();

    let mut min_m = input[0];
    let mut min_idx = 0;
    let mut start_nums = vec![];

    for i in 1..input.len(){
        if input[i] == -1 { continue; }
        let modulo = input[0] % input[i];
        if modulo == 0 {
            min_idx = i;
            min_m = 0;
        //    break;
        } 
        else if input[i] - modulo < min_m {
            min_m = input[i] - modulo;
            min_idx = i;
        }
        let diff =  (i as i64 - 1)  % input[i];
        start_nums.push((input[i], diff));        
    }
    println!("star1: {}", input[min_idx] * min_m);

    start_nums.sort_by_key(|k| k.0);
    start_nums.reverse();
    //star2
    let ref_id = start_nums[0].0;
    let ref_diff = start_nums[0].1;
    let earliest_timestamp : i64;
    if input[0] == 1000390{  
        earliest_timestamp  = 100000000000000;
    } else {
        earliest_timestamp = ref_id;
    }

    let mut i = (earliest_timestamp / ref_id) as i64;
    let mut step = 1;
    let mut last_id = ref_id;
    loop {
        let mut fin = true;
        for e in 1..start_nums.len(){
            let id_ = start_nums[e].0;
            let diff = start_nums[e].1;
            let modulo = (ref_id * i + diff- ref_diff) % id_;
            if modulo  != 0 {
                fin = false;
                break;
            } else {
                if last_id > id_ {
                    step *= id_;
                    last_id = id_;
                  //  println!("{} {} {} {}", id_, i, step, last_id);
                }
            }
        }
        if fin { break; }
        i += step;
        if i * ref_id > 100000000000000 * 6 { println!("failed"); break;}
    }
    println!("star2: {} ",(i*ref_id-ref_diff));
    //println!("star2: {} (at index {})",(i*ref_id-ref_diff),i);
    //println!("star2: {} ({})",(i*ref_id + ref_start) * input[1],i);
    //3093856321931817 too high
}
