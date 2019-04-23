use std::env;
use std::fs::File;
use std::io::prelude::*;


fn length(s: &String)->i32{
    let mut cnt = 0;
    for c in s.chars().enumerate(){
        if c.1 != ' '{
            cnt+=1;
        }
    }
    cnt 

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split_str = contents.split("\n");
    for s in split_str {
        println!("");
        let mut start = 0;
        let mut decomp = String::new();
        loop {
            if start >= s.len(){
                break;
            }

            let m1 = s[start..].find('(');
            if m1 == None {
                decomp += &s[start..]; 
                break;
            } 

            let p1 = m1.unwrap() + 1 + start;
            if p1 > start+1 {
                decomp += &s[start..p1-1]; 
            }
            let m2 = s[p1..].find(')');
            if m2 == None {
                assert!(false);
            }
            let p2 = m2.unwrap() + p1;
            start = p2 + 1; 

            let tmp = &s[p1..p2];
            println!("{}", tmp);
            let args = tmp.split("x").collect::<Vec<&str>>();
            let num_chars = args[0].parse::<usize>().unwrap(); 
            let num_times = args[1].parse::<usize>().unwrap(); 
   
            for i in 0..num_times{
                decomp += &s[start..start+num_chars]; 
            }

            start += num_chars;

            println!("{} -> {} x {}   < {} >", tmp, num_chars, num_times, decomp);
             
        }
        println!("decompressed string: {} , length: {}", decomp, length(&decomp));

    }
}
