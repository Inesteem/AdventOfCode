extern crate lib;

use lib::io::*;
use std::process::exit;

fn main() {
    let mut cnt = 0;
    let mut star2 = 0;
    let mut v= vec![false;27];
    let mut v2= vec![0;27];
    let mut team_people = 0;

    loop {
       let line;
       match read_line_from_stdin() {
            Ok(l) => line =l,
            Err(_) => {println!("error while reading stdin"); exit(-1); },
       }
        
       if line.len() <= 1 {
            for i in 0..v.len() {
                if v[i] {
                    cnt += 1;
                    v[i] = false;
                }
            }
            for i in 0..v2.len() {
                if v2[i] == team_people{
                    star2 += 1;
                }
                v2[i] = 0;
            }
            team_people = 0;

       } else { team_people += 1; }

       if line.len() == 0 { break; }//EOF
       
       for c in line[..line.len()-1].chars(){
           let pos = (c as u32) - ('a' as u32);
           v[pos as usize] = true;
           v2[pos as usize] += 1;
       }

    }
    println!("star1 : {}", cnt);
    println!("star2 : {}", star2);
}
