use std::env;
use std::char;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split_str = contents.split("\n");

    let codelen = contents.find('\n').unwrap();

    let mut code = vec![[0u8; 27]; codelen];
//    println!("With text:\n{}", contents);
    for sequence in split_str {
        //println!("{}", s)
        for (i,s) in sequence.chars().enumerate(){
            let idx: usize = (s.to_digit(36).unwrap()-10) as usize;
            //println!("{:?}", code[i][idx])
            code[i][idx] += 1;
        }
        //println!("");
    }

//    for c in &mut code {
//        println!("{:?}", c);
//    }
    print!("\nthe message is : ");

    for x in 0..codelen {

        let max = code[x].iter().max().unwrap();
        let index = (code[x].iter().position(|element| element == max).unwrap()) as u32;
        //println!("idx : {}", index);

        let c = char::from_digit(index+10, 36);
        print!("{}", c.unwrap());
    }
    println!("");
}

