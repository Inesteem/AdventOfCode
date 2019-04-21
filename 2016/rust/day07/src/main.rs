use std::env;
use std::fs::File;
use std::io::prelude::*;

fn check_ip(s: &str) -> bool {
    //let char_vec: Vec<char> = s.chars().collect();
    //for c in char_vec {
    //    println!("{}", c);
    //}
    let tmp = s.split("").collect::<Vec<&str>>();
    if tmp.len() < 4{
        return false;
    }
//    println!("{:?}", tmp);
//    println!("{}", tmp.len());

    let mut abba = false;
    let mut in_bracks = false;

    for i in 1..tmp.len()-4{
        if tmp[i] == "["{
            in_bracks = true;
            continue; 
        }
        if tmp[i] == "]"{
            in_bracks = false;
            continue; 
        }
        if tmp[i] == tmp[i+3]{
            if tmp[i+1] == tmp[i+2] && tmp[i] != tmp[i+1]{
                if in_bracks{
                    return false;
                }
                abba = true;
            } 
        } 
    }


    abba
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split_str = contents.split("\n");

    let mut cnt = 0;

    for s in split_str {

        if check_ip(s) == true {
            cnt += 1;
        }
        
    }
    println!("{} IPs support TLS!", cnt );
}
