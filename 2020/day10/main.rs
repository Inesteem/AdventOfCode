extern crate lib;
use lib::io::*;


fn main(){
    let mut jolts = vec![0];
    loop {
        let line = read_line_from_stdin().unwrap();

        if line.len() == 0 {break;}

        jolts.push(line[0..line.len()-1].parse::<i32>().unwrap());
    
    }
    jolts.sort();
    jolts.push(jolts[jolts.len()-1]+3);
    println!("{:?}", jolts);


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

    println!("{} * {} = {}", diff1, diff3, diff1*diff3);
}
