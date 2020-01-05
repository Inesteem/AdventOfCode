use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Disc {
    start_pos: usize,
    num_pos: usize,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split_str = contents.split("\n");

    let mut discs = Vec::new();

    for s in split_str {
        let info = s.split(" ").collect::<Vec<&str>>();
        if info.len() < 10 {continue;}
    
        discs.push( Disc {       
            num_pos:   info[3].parse::<usize>().unwrap(),
            start_pos: info[11].parse::<usize>().unwrap(),
        });
    }

    //start 2
    discs.push( Disc {       
        num_pos:   11,
        start_pos: 0,
    });


    for i in 0..std::usize::MAX {
        let mut cnt = 0;
        for d in 0..discs.len() {
            cnt += (i + d + 1 + discs[d].start_pos) % discs[d].num_pos;
        }
        if cnt == 0 {
            println!("{}", i);
            break;
        }

    }
}
