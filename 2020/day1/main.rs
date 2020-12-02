use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    //println!("{}", &contents);
    Ok(contents)
}




fn main() {
    let mut numbers: Vec<u32>;
    match read_inputs("dat".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            numbers = inputs.split("\n")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect(),
        Err(_) => process::exit(0),
    }
    numbers.sort();
    println!("{:?}", &numbers);
    for i in 0..numbers.len() {
      let mut add;
      for j in 0..numbers.len() {
        if i == j {
            continue;
        }
        add = numbers[i] + numbers[j];
        if add > 2020 {
            break;
        }
       // if add == 2020 { 
       //     println!("first star: {}", numbers[j]*numbers[i]);
       //     process::exit(0);
       //}
        for k in 0..numbers.len() {
            if i == k || j == k {
                continue;
            }
            if add + numbers[k] > 2020 {
                break;
            }
            if add  + numbers[k]== 2020 {
              println!("second star: {}", numbers[j]*numbers[i]*numbers[k]);
              process::exit(0);
            }
        }
      } 
    } 
}
