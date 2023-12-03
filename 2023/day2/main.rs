use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::max;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_num(s: &str, re: &regex::Regex) -> i32 {
    let Some(num) = re.captures(s) else {
        return -1;
    };
    return num["num"].parse::<i32>().unwrap();
}

fn main() {
    let re = Regex::new(r"(?<num>[0-9]+)").unwrap();

    let files = vec!["test", "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }



        let lines : Vec<&str> = input.lines().collect();
        let mut ids :i32 = 0;
        for line in lines {

            let mut cube_map: HashMap<&str, i32> = [
                ("red", 12),
                ("green", 13),
                ("blue", 14),
            ].iter().cloned().collect();

            let game: Vec<&str>= line.split(":").collect();
            let split: Vec<&str>= game[1].split(";").collect();
            for round in split {
                let cubes: Vec<&str>= round.split(",").collect();
                for cube in cubes {
                    let cube_split: Vec<&str>= cube.split(" ").collect();
                    let num = cube_split[1].parse::<i32>().unwrap();
                    let entry=*cube_map.entry(cube_split[2]).or_insert(num);
                    cube_map.insert(cube_split[2], max(num, entry));

                }
            }
            if cube_map["red"] > 12|| cube_map["green"] > 13 || cube_map["blue"] > 14 {
                continue;
            }
            ids +=  get_num(game[0], &re);
            
        }
        //231 too low
        println!("{}", ids);
    }
}
