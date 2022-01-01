use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::{max,min};

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_kms_at(speed: usize, secs: usize, rest: usize, at_sec : usize) -> usize {
    let full_cycles = at_sec / (secs+rest);
    let kms = full_cycles * secs * speed;
    kms + min(at_sec - full_cycles * (secs + rest), secs) * speed
}

fn main() {
    let files = vec!["test", "data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let seconds = 2503;
        //let seconds = 1000;

        let lines : Vec<Vec<&str>>= input.lines().map(|line| line.split_whitespace().collect()).collect();
        let star = 2;
        if star == 1 {
            let mut kilometres = 0;
            for line in &lines {
                let speed : usize = line[3].parse().unwrap();
                let secs  : usize = line[6].parse().unwrap();
                let rest : usize = line[13].parse().unwrap();

                let kms = get_kms_at(speed, secs, rest, seconds);
                kilometres = max(kilometres, kms);
                println!("{} {}", line[0], kms);
            }
            println!("Best Reindeer: {}\n", kilometres);
        } else {
            let mut reindeer_kms : Vec<Vec<usize>> = vec![vec![0;seconds]; lines.len()];
            let mut farest : Vec<usize> = vec![0;seconds];

            for i in 0..reindeer_kms.len() {
                let line = &lines[i];
                let speed : usize = line[3].parse().unwrap();
                let secs  : usize = line[6].parse().unwrap();
                let rest : usize = line[13].parse().unwrap();
                
                for second in 0..seconds{

                    reindeer_kms[i][second] = get_kms_at(speed, secs, rest, second+1);
                    farest[second] = max(farest[second], reindeer_kms[i][second]);
                }
            }

            let mut points : Vec<usize> = vec![0;lines.len()];
            for s in 0..seconds {
                for r in 0..lines.len() {
                    if reindeer_kms[r][s] == farest[s] {
                        points[r] += 1;
                    }
                }
            }
            let mut max_points = 0;
            for p in &points {
                max_points = max(*p, max_points);
            }
            println!("{:?}", points);
            println!("{}", max_points);
        }
    }
}
