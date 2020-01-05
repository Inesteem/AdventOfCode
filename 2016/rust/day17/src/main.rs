use::std::fs::File;
use::std::io::{BufRead, BufReader};
use::std::env;

extern crate crypto;
extern crate hex;

use crypto::md5::Md5;
use crypto::digest::Digest;

mod helper;
use helper::*;

//(x,y)
const START:(i8, i8) = (1,1);
const GO_L: ((i8, i8), char) = ((-1,0),'L');
const GO_R: ((i8, i8), char) = ((1,0) ,'R');
const GO_U: ((i8, i8), char) = ((0,-1),'U');
const GO_D: ((i8, i8), char) = ((0,1) ,'D');
static DIR: &'static [((i8,i8),char)] = &[GO_U, GO_D, GO_L, GO_R];


fn add_tuple((t11, t12): &(i8,i8),(t21, t22): &(i8,i8))->(i8,i8){
    (t11+t21, t12+t22)

}

fn elem_at((x,y): &(i8,i8), board: &Vec<Vec<char>>) -> char{
    board[*y as usize][*x as usize]
}
 

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} [hash_val]", args[0]);
        std::process::exit(-1);
    } 
    let mut hasher = Md5::new();
    let key = &args[1].as_bytes();
    

    let filename = "../../input/day17.dat";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut board = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    //for (index, line) in reader.lines().enumerate() {
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.

        let char_vec: Vec<char> = line.chars().collect();
        board.push(char_vec);
    }


    let mut way: Vec<char> = Vec::new();
    //i, pos, way
    let mut checkpoints: Vec<(usize, (i8,i8), Vec<char>)> = Vec::new();

    let mut pos: (i8 , i8) = START;

    //let mut finished = false;

    let mut ways: Vec<String> = Vec::new();
    let mut s_len : usize = std::usize::MAX;
    let mut l_len : usize = 0;
    loop {
//        println!("");
        hasher.input(key);
        let way_str : String = way.iter().collect();
        if elem_at(&pos, &board)== 'V'{
//            println!("way found : {}", way_str);
            ways.push(way_str.clone());
            s_len = way.len();
            if way.len() > l_len {
                println!("new maxlen: {}", l_len);
                l_len = way.len();
            }
        } else { 
            hasher.input(way_str.as_bytes());
            let mut output = [0; 16]; // An MD5 is 16 bytes
            hasher.result(&mut output);

            //debug output
            let first_four : String = hex::encode(output).chars().into_iter().take(4).collect();
            let dec : Vec<u32>=  first_four.chars().map(|c| c.to_digit(16).unwrap()).collect();
    //        println!("> {:?}", dec);
    //        println!("{} - pos: {:?} - way: {}", first_four, pos, way_str);
    //        println!("{:?}", output);
    //        println!("{:?}", hex::encode(output));
            for i in 0..4 {
                if dec[i] > 10 {
                    let n_f = add_tuple(&(DIR[i].0), &pos);
                    let free = elem_at(&n_f, &board) != '#';            
                    if free {
    //                    println!("{}: door open -> go: {:?}; next pos: {:?}", output[i], DIR[i], pos);
                        checkpoints.push((i, pos, way.clone()));
                    } else {
    //                    println!("{}: no door {:?}", output[i], DIR[i]);
                    }
                } else {
    //                println!("{}: door closed!", output[i]);    
                }
            }
        }
        if checkpoints.len() == 0 { 
//            println!("way found:");
            break;
        }
       // loop {
       //     println!("{:?}", ways);
       //     if checkpoints.len() == 0 { 
       //         println!("way found:");
       //         
       //         for s in ways {
       //             println!("{}", s);
       //         }
       //         std::process::exit(-1);
       //     }
            //use last checkpoint
            let (i, pos_tmp, way_tmp) = checkpoints.pop().unwrap();
            let n_f = add_tuple(&(DIR[i].0), &pos_tmp);
            pos = add_tuple(&(DIR[i].0), &n_f);
            way = way_tmp;
            way.push(DIR[i].1);
       //     if way.len() <= s_len { break};
       // }
        hasher.reset();
        //pause(); 
    }

    println!("{}", l_len);

//    for line in &board {
//        let res: String = line.iter().collect();
//        println!("{}", res);
//    }

}
