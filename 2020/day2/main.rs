use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::io;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    //println!("{}", &contents);
    Ok(contents)
}

fn get_challenge() -> u8 {
    print!("which challenge (0 for first star, any other number: 1");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();
    if n != 0 {
        return 1;
    }
    n
}


fn main() {
    let inputs: Vec<String>;
    match read_inputs("dat".to_string()) {
        Ok(i) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            inputs = i.split(&['-', ':', '\n', ' '][..])
            .map(|x| x.to_string())
            .collect(),
        Err(_) => process::exit(0),
    }
    println!("{:?}", &inputs);
    let challenge = get_challenge(); 
    println!("");
    let mut valid = 0;
    for i in (0..inputs.len()-4).step_by(5) {
        let min : usize= inputs[i].parse().expect("Not an integer!");
        let max : usize= inputs[i+1].parse().expect("Not an integer!");
        let val = inputs[i+2].chars().nth(0).unwrap();
        let pwd =  &inputs[i+4];
        let mut num = 0;

        for c in pwd.chars() {
            if c == val { num+=1;}
        } 
        if challenge == 0 && num >= min && num <= max {
            valid+=1;
        }
        if challenge != 0 {
            let c_pos1 = pwd.chars().nth(min-1).unwrap();
            let c_pos2 = pwd.chars().nth(max-1).unwrap();
            //println!("{} - {},{}", &pwd, c_pos1==val, c_pos2==val);
            if (c_pos1 == val && c_pos2 != val) || (c_pos1 != val && c_pos2 == val){
                valid+=1;
                //println!("{} - {},{},{}", min, max, val , &pwd);
            }
        }
    }
    println!("number of valid pws: {}", valid);
}
