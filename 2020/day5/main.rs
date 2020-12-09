use std::process;
extern crate lib;
use lib::io::*;


fn get_seat_id(line: &str) -> i16 {
    let mut minR : i16 = 0;
    let mut maxR : i16 = 128;
    let mut minC : i16 = 0;
    let mut maxC : i16 = 8;
    let mut row : i16 = -1;
    let mut col : i16 = -1;
    for c in line.chars(){
        if c == 'F' {
            maxR -= (maxR - minR) / 2;
            row = minR;
        }
        else if c == 'B' {
            minR += (maxR - minR) / 2;
            row = minR;
        }
        else if c == 'L' {
            maxC -= (maxC - minC) / 2;
            col = minC;
        }
        else if c == 'R' {
            minC += (maxC - minC) / 2;
            col = minC;
        }//TODO: else?
        //println!("{} -> {} {}  |  {} {}",c, minR, maxR, minC,maxC);
    }
    row * 8 + col
}

fn main() {
    let mut highest = 0;
    let mut v = vec![false; 128 * 8];
    loop{
        let mut line;
        match read_line_from_stdin() {
            Ok(l) => line = l,
            Err(_) => {println!("error while reading stdin") ;process::exit(0);},
        }
        if line.len() == 0 {
            break;
        }
        //println!("{}",&line[..line.len()-1]);
        let seatID = get_seat_id(&line[..]);
        v[seatID as usize] = true;
        //println!("id: {}",seatID);
        if seatID> highest { highest = seatID; }

    }
    println!("star1: {}",highest);

    for i in 8..v.len()-8{
        if !v[i] && (v[i-1] && v[i+1]) {
            println!("star2: {}",i);
        }
    }
}

