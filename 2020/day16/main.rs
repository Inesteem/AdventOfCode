use lib::io::*;

#[derive(Debug)]
struct Range {
    start : usize,
    end : usize,
}
impl Range {
    fn new(i : &str) -> Self {
        let intervals : Vec<&str> = i.split('-').collect();
        let start = intervals[0].parse::<usize>().unwrap();
        let end = intervals[1].parse::<usize>().unwrap();
        Self {
            start : start,
            end : end,
        }
    }
    fn in_range(&self, val : usize) -> bool {
        return val >= self.start && val <= self.end;
    }
}

#[derive(Debug)]
struct Field {
    name : String,
    ranges : Vec<Range>,
}

impl Field {
    
    fn new(line : &Vec<String>) -> Self{
        Self {
            ranges : vec![Range::new(&line[1]),Range::new(&line[2])],
            name : line[0].clone(),
        }
    }
    fn in_range(&self, val : usize)->bool {
        for r in &self.ranges {
            if r.in_range(val) {
                return true;
            }
        }
        false  
    }
}

fn remove_from(needle : &str, haystack : &mut Vec<Vec<&str>>){
    for l in haystack {
        if l.len() <= 1 { continue; }
        l.retain(|&x| !x.eq(needle));
    }
}



fn star2(list : &mut Vec<Vec<String>>){

    let mut fields = vec![];
    let mut yt_idx = 0;
    while !list[yt_idx][0].eq("-") {
        fields.push(Field::new(&list[yt_idx]));
        yt_idx += 1;
    }

    yt_idx += 2;

    let mut possible_fields : Vec<Vec<&str>> = vec![];
    
    for col in 0..fields.len() {
        possible_fields.push(Vec::new());
        for f in 0..fields.len(){

            let mut valid = true;
            let field = &fields[f]; 
            
            for row in yt_idx..list.len() {
                if !field.in_range(list[row][col].parse::<usize>().expect("not a number")) {
                    valid = false;
                    break;
                } 
            }
            if valid {
                possible_fields[col].push(&field.name[..]);
            }
        }
    }


    for _i in 0..100 {
        for i in 0..possible_fields.len() {
            if possible_fields[i].len() == 1 {
                remove_from(&possible_fields[i][0], &mut possible_fields);
            }
        }
    }

    let mut mul = 1;
    for f in 0..possible_fields.len() {
        if possible_fields[f][0].len() >= "departure".len() &&  possible_fields[f][0][0.."departure".len()].eq("departure") {
            mul *= list[yt_idx-1][f].parse::<usize>().unwrap();
        }

    }
    println!("star2: {}", mul);
}

fn add_range(l : &str,ranges : &mut Vec<bool>) {
    let intervals : Vec<&str> = l.split('-').collect();
    let start = intervals[0].parse::<usize>().unwrap();
    let end = intervals[1].parse::<usize>().unwrap();
    if ranges.len() <= end {
        ranges.resize(end+1,false);
    }
    for i in start..=end {
        ranges[i] = true;
    }
}

fn star1()-> Vec<Vec<String>>{
    let mut ranges = vec![];
    let mut line = "".to_string();
    let mut list = vec![];
    loop {
        line = read_line_from_stdin().unwrap().trim().to_string();
        if line.len() == 0 { continue; }
        if line.eq("your ticket:") { break; }
        let split1: Vec<&str> = line.split(':').collect();
        let split2: Vec<&str> = split1[1].split(' ').collect();
        add_range(split2[1], &mut ranges);
        add_range(split2[3], &mut ranges);
        list.push(vec![split1[0].to_string(), split2[1].to_string(), split2[3].to_string()]);
    }
    list.push(vec!["-".to_string()]);

    loop {
        line = read_line_from_stdin().unwrap().trim().to_string();
        if line.len() == 0 { continue; }
        if line.eq("nearby tickets:") { break; }
        let split: Vec<String> = line.split(',').map(|x| x.to_string()).collect();
        list.push(split);
    }

    let mut sum = 0;
    loop {
        line = read_line_from_stdin().unwrap().trim().to_string();
        if line.len() == 0 {break; }
        let splitted : Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let mut sane = true;
        for s in splitted {
            if s >= ranges.len() || !ranges[s] { sum += s; sane = false; }
        }
        if sane {
        list.push(line.split(',').map(|x| x.to_string()).collect());
        }
    }
    println!("star1: {}", sum);
    list
}

fn main() {
    let mut list = star1();
    star2(&mut list);
}
