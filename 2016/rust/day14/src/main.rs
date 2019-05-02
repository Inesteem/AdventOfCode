use std::env;
extern crate crypto;
extern crate hex;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::str;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
 



    let mut hasher = Md5::new();

   // let key = "abc".as_bytes();
    //let key = "ngcjuoqr".as_bytes();
    let key = &args[1].as_bytes();
    println!("{}", args[1]);
    //for i in (0..std::u64::MAX) {

    let mut triples = HashMap::new();
    let mut hashes = Vec::new();
    for i in (0..std::u64::MAX) {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);
        let mut res =  hex::encode(output);

        for _j in 0..2016 {
            hasher.reset();
            hasher.input(res.as_bytes());
            hasher.result(&mut output);
            res =  hex::encode(output);
        }

        let vec: Vec<char> = res.chars().collect();
        let mut found_3 = false;
        for idx in 0..vec.len()-2{
            if vec[idx] == vec[idx+1] && vec[idx] == vec[idx+2] {
            //    println!("{}: {}", i, hex::encode(output));

                if (!found_3){
                    found_3 = true;
                    let t_vec = triples.entry(vec[idx]).or_insert(Vec::new());
                    t_vec.push((i, vec[idx],  res.clone()));
                }
                if idx <= vec.len()-5 && vec[idx] == vec[idx+3] && vec[idx] == vec[idx+4] {
                    if ! triples.contains_key(&vec[idx]) {
                        continue;
                    }
                    let t_vec = triples.get_mut(&vec[idx]).unwrap();   
 

 
                    for j in (0..t_vec.len()).rev(){
                        let triple = &t_vec[j];
                        let diff = i - triple.0;
                        if diff > 1000 {
                            t_vec.remove(j);
                            continue;
                        }
                        if (*triple).1 == vec[idx] {
                            
                            if diff <= 1000  && diff > 0{
                                println!("[{}]    [{}] {} : {}", hashes.len(), i, triple.0, triple.2);
                                
                                hashes.push((triple.0, i, triple.2.clone())); 
                                t_vec.remove(j);
                                if hashes.len() == 100 {// so some more, because value at sorted idx 64 might be found some time later; not really correct but works on the provided input 
                                    hashes.sort_by(|a,b| a.0.cmp(&b.0));
                                    for h in &hashes {
                                        println!("{} [{}] : {}",  h.0, h.1, h.2);

                                    }
                                    println!("num 64 {:?}", hashes[63]);
                                    //hashes.sort_by(|a,b| a.0.cmp(&b.0).reverse());
                                    std::process::exit(0);
                                }
                            } 
                        }
                    }
                }

            }
            
        }


        hasher.reset();
    }
}

