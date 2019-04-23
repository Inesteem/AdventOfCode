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

fn get_len(s: &str)->i32{
    let mut length = 0;
    let mut start = 0;
    loop {
        if start >= s.len(){
            return length;
        }

        let m1 = s[start..].find('(');
        if m1 == None {
            if start == 0 {
                return s.len();
            }
            return length + s.len() - start;
        } 

        let p1 = m1.unwrap() + 1 + start;
        if p1 > start+1 {
            length += (p1 - 1 - start); 
        }
        let m2 = s[p1..].find(')');
        if m2 == None {
            assert!(false);
        }
        let p2 = m2.unwrap() + p1;
        start = p2 + 1; 

        let tmp = &s[p1..p2];
        let args = tmp.split("x").collect::<Vec<&str>>();
        let num_chars = args[0].parse::<usize>().unwrap(); 
        let num_times = args[1].parse::<usize>().unwrap(); 
        
        length += get_len(s[start..start+num_chars]) * num_times; 

        start += num_chars;
             
    }
 

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
         
        println!("length: {}", get_len(s));
        //println!("decompressed string: {} , length: {}", decomp, length(&decomp));

    }
}
