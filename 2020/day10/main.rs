extern crate lib;
use lib::io::*;
use std::collections::HashMap;

fn num_ways(last_elem : u32, pos : usize, jolts : &Vec<u32>, mem : &mut HashMap<String,usize>) -> usize{
    if pos >= jolts.len() { return 0; }
    if jolts[pos] - last_elem > 3 { return 0;}
    if pos == jolts.len()-1 { return 1; }

    let mut key = String::from(last_elem.to_string());
    key.push_str("-");
    key.push_str(&pos.to_string());

    if mem.contains_key(&key){
        return *mem.get(&key).unwrap();
    }

    let num = num_ways(jolts[pos], pos+1,jolts, mem) +
              num_ways(jolts[pos], pos+2, jolts, mem) +
              num_ways(jolts[pos], pos+3, jolts, mem);
    mem.insert(key,num);
    num
}

fn main(){
    let mut mem = HashMap::<String,usize>::new();
    let mut jolts = vec![0];
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 {break;}
        jolts.push(line[0..line.len()-1].parse::<u32>().unwrap());
    
    }
    jolts.sort();
    jolts.push(jolts[jolts.len()-1]+3);

    //star 1
    let mut diff1 = 0;
    let mut diff3 = 0;
    for i in 1..jolts.len() {
        match jolts[i] - jolts[i-1]{
            1 => diff1 +=1,
            3 => diff3 +=1,
            _ => (),
        };
    }

    println!("star1 = {} * {} = {}", diff1, diff3, diff1*diff3);

    let ways = num_ways(0,0,&jolts,&mut mem);

    println!("star2: {}", &ways);
}
