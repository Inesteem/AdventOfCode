use std::collections::HashMap;

fn main() {
    //let start_sequence = [0,3,6];
    let start_sequence = [0,13,16,17,1,10,6];
    let mut i : i32 = 0;
    let mut numbers = HashMap::<i32,i32>::new();

    while (i as usize) < start_sequence.len()-1 {
        //println!("{} {}",i + 1,start_sequence[i as usize]);
        numbers.insert(start_sequence[i as usize],i);
        i += 1;
    }
    let mut last_num = start_sequence[start_sequence.len()-1];

    while i < 2019 {
        //for n in &numbers { print!("{:?}", n);}
        //println!("{} {}",i + 1, last_num);
        if numbers.contains_key(&last_num) {
            //let last_time = numbers.get(&last_num).unwrap().clone();
            let last_time = numbers.remove(&last_num).unwrap();
            numbers.insert(last_num,i);
            last_num = i - last_time;
           // println!("  {} {}", i, last_time);
            
        } else {
            numbers.insert(last_num,i);
            last_num = 0;
            //println!("");
        }
        i += 1;
    }
    println!("star1: {}",last_num);
}
