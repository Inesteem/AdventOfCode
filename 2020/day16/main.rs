use lib::io::*;

fn add_range(l : &str,ranges : &mut Vec<bool>) {
    let intervals : Vec<&str> = l.split('-').collect();
    println!("{:?}",intervals);
    let start = intervals[0].parse::<usize>().unwrap();
    let end = intervals[1].parse::<usize>().unwrap();
    if ranges.len() <= end {
        ranges.resize(end+1,false);
    }
    for i in start..=end {
        ranges[i] = true;
    }
}

fn star1(){
    let mut ranges = vec![];
    let mut line = "".to_string();

    loop {
        line = read_line_from_stdin().unwrap().trim().to_string();
        println!("{}",line);
        if line.len() == 0 { continue; }
        if line.eq("your ticket:") { break; }
        let split1: Vec<&str> = line.split(':').collect();
        let split2: Vec<&str> = split1[1].split(' ').collect();
        println!("- {:?}", split1);
        println!("- {:?}", split2);
        add_range(split2[1], &mut ranges);
        add_range(split2[3], &mut ranges);
    }


    while !line.eq("nearby tickets:\n"){
        line = read_line_from_stdin().unwrap();
    }
    let mut sum = 0;
    loop {
        line = read_line_from_stdin().unwrap().trim().to_string();
        println!("{}",line);
        if line.len() == 0 {break; }
        let splitted : Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        for s in splitted {
            if s >= ranges.len() || !ranges[s] { sum += s; }
        }
    }
    //println!("{:?}", ranges);
    println!("star1: {}", sum);
}

fn main() {
    star1();
}
