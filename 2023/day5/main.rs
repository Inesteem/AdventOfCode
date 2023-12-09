use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

struct Mapping {
    src_start: i64,
    dst_start: i64,
    range_len: i64,
}

impl Mapping {
    fn new(s :i64, d:i64, l:i64)->Mapping {
        Mapping{
            src_start : s,
            dst_start: d,
            range_len: l,
        }
    }
}

struct Mappings {
    mappings : Vec<Mapping>,
}

impl Mappings {
    fn new() -> Mappings {
        Mappings {mappings : vec![]},
    }

    fn add(&self, s:i64, d:i64, l:i64){
        self.mappings.push(Mapping::new(s, d, l);
    }
}

fn read_inputs(filename: String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_num(s: &str) -> i64 {
    return s.parse::<i64>().unwrap();
}

fn get_vec(line: &str) -> Vec<i64> {
    return line
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| get_num(x))
        .collect();
}
fn fill_map(lines: &Vec<&str>, i: &mut usize, map: &mut HashMap<i64, i64>) {
    while *i < lines.len() && !lines[*i].is_empty() {
        let line = &lines[*i];
        let nums: Vec<i64> = get_vec(line);
        assert!(nums.len() >= 3);
        let dest_range_start = nums[0];
        let source_range_start = nums[1];
        let range_len = nums[2];
        for l in 0..range_len {
            map.insert(source_range_start + l, dest_range_start + l);
        }
        *i += 1;
        println!("{}: {:?}", *i, nums);
    }
}
fn get_dest(src: i64, map: &mut HashMap<i64, i64>) -> &mut i64 {
    return map.entry(src).or_insert(src);
}

fn main() {
    let files = vec!["test", "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) => input = inputs,
            Err(_) => process::exit(0),
        }

        let lines: Vec<&str> = input.lines().collect();
        let mut seeds: Vec<i64> = vec![];
        let mut seed_to_soil: HashMap<i64, i64> = HashMap::new();
        let mut soil_to_fertilizer: HashMap<i64, i64> = HashMap::new();
        let mut fertilizer_to_water: HashMap<i64, i64> = HashMap::new();
        let mut water_to_light: HashMap<i64, i64> = HashMap::new();
        let mut light_to_temperature: HashMap<i64, i64> = HashMap::new();
        let mut temperature_to_humidity: HashMap<i64, i64> = HashMap::new();
        let mut temperature_to_humidity: HashMap<i64, i64> = HashMap::new();
        let mut humidity_to_location: HashMap<i64, i64> = HashMap::new();

        let mut i = 0;
        while i < lines.len() {
            let line = &lines[i];
            if line.len() == 0 {
                continue;
            }
            let split: Vec<&str> = line.split(":").collect();
            if split.len() > 1 {
                i += 1;
                match split[0] {
                    "seeds" => seeds = get_vec(split[1]),
                    "seed-to-soil map" => fill_map(&lines, &mut i, &mut seed_to_soil),
                    "soil-to-fertilizer map" => fill_map(&lines, &mut i, &mut soil_to_fertilizer),
                    "fertilizer-to-water map" => fill_map(&lines, &mut i, &mut fertilizer_to_water),
                    "water-to-light map" => fill_map(&lines, &mut i, &mut water_to_light),
                    "light-to-temperature map" => {
                        fill_map(&lines, &mut i, &mut light_to_temperature)
                    }
                    "temperature-to-humidity map" => {
                        fill_map(&lines, &mut i, &mut temperature_to_humidity)
                    }
                    "humidity-to-location map" => {
                        fill_map(&lines, &mut i, &mut humidity_to_location)
                    }
                    _ => {
                        println!("{}", split[0]);
                        break;
                    }
                }
            }
            i += 1;
        }
        let mut location = std::i64::MAX;
        for seed in seeds {
            let mut val = get_dest(seed, &mut seed_to_soil);
            val = get_dest(*val, &mut soil_to_fertilizer);
            val = get_dest(*val, &mut fertilizer_to_water);
            val = get_dest(*val, &mut water_to_light);
            val = get_dest(*val, &mut light_to_temperature);
            val = get_dest(*val, &mut temperature_to_humidity);
            val = get_dest(*val, &mut humidity_to_location);
            println!("seed {} -> {}", seed, val);
            location = min(location, *val);
        }
        println!("star1: {}", location);
    }
}
