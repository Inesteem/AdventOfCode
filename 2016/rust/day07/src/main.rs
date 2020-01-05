use std::env;
use std::fs::File;
use std::io::prelude::*;


fn check_bab(s: &str, c_o: &str, c_i: &str, i1: usize, i2: usize) -> bool{
    if c_i == c_o{ 
        return false;
    }
    //println!("{} ->  {}, {}", s,c_o,c_i); 

    let tmp = s.split("").collect::<Vec<&str>>();
    for i in i1+1..i2-2{
        println!("--- {}:{}", i,tmp[i]);
        if tmp[i] == c_o && tmp[i+2] == c_o && tmp[i+1] == c_i{
            return true;
        }
    }
    false 
}


fn check_ssl(s: &str) -> bool {
    println!("{}", s);
    let mut start = 0;

    loop {
        println!("--- {}", start);
        let p1 = s[start..].find('[');
        println!("{}", &s[start..]);
        if p1 == None{
            return false;
        }
        let i1 = start+p1.unwrap()+1;
        let p2 = s[i1..].find(']');
        println!("{}", &s[i1+1..]);
        if p2 == None{
            return false;
        }
        let i2 = i1+p2.unwrap()+1;
        println!("{} .. {}", i1-1, i2-1);
        start = i1;
        let vec = s.split("").collect::<Vec<&str>>();

        let mut in_bracks = false; 
        for i in 1..vec.len()-2{
            if vec[i] == "["{
                in_bracks = true;
                continue;
            }
            if vec[i] == "]"{
                in_bracks = false;
                continue;
            }
            if in_bracks {
                continue;
            }
            if vec[i+1] == "[" || vec[i+2] == "["{
                continue;
            }
            println!("- {}:{}", i,vec[i]);
            if vec[i] == vec[i+2] && check_bab(s, vec[i+1], vec[i], i1, i2){
                println!("found: {}{}{}",vec[i], vec[i+1], vec[i+2]); 
                return true;
            }
        }
        
    }

}


fn check_tls(s: &str) -> bool {
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

    let mut tls_cnt = 0;
    let mut ssl_cnt = 0;

    for s in split_str {

        if check_tls(s) == true {
            tls_cnt += 1;
        }
        if check_ssl(s) == true {
            ssl_cnt += 1;
        }
        
    }
    println!("{} IPs support TLS!", tls_cnt );
    println!("{} IPs support SSL!", ssl_cnt );
}
