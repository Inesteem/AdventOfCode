use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

// Note: incomplete. My laptop crashed and I lost the rust code. Frustrated I started over in
// python, this is some weird state I saved in git for random reasons. "Almost" correct, but still
// broken somehow.

struct Mapping {
    src_start: i64,
    dst_start: i64,
    range_len: i64,
}

impl Mapping {
    fn new(s: i64, d: i64, l: i64) -> Mapping {
        Mapping {
            src_start: s,
            dst_start: d,
            range_len: l,
        }
    }

    fn maps(&self, s: i64) -> bool {
        return s >= self.src_start && s < (self.src_start + self.range_len);
    }

    fn map(&self, s: i64) -> i64 {
        return self.dst_start + (s - self.src_start);
    }
}

struct Mappings {
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn new() -> Mappings {
        Mappings { mappings: vec![] }
    }

    fn add(&mut self, s: i64, d: i64, l: i64) {
        self.mappings.push(Mapping::new(s, d, l));
    }

    fn entry(&self, s: i64) -> i64 {
        for m in &self.mappings {
            if m.maps(s) {
                return m.map(s);
            }
        }
        return s;
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
fn fill_map(lines: &Vec<&str>, i: &mut usize, map: &mut Mappings) {
    while *i < lines.len() && !lines[*i].is_empty() {
        let line = &lines[*i];
        let nums: Vec<i64> = get_vec(line);
        assert!(nums.len() >= 3);
        let dest_range_start = nums[0];
        let source_range_start = nums[1];
        let range_len = nums[2];
        map.add(source_range_start, dest_range_start, range_len);
        *i += 1;
        println!("{}: {:?}", *i, nums);
    }
}
fn get_dest(src: i64, map: &Mappings) -> i64 {
    return map.entry(src);
}

fn main() {
    let files = vec!["test"]; //, "input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) => input = inputs,
            Err(_) => process::exit(0),
        }

        let lines: Vec<&str> = input.lines().collect();
        let mut seeds: Vec<i64> = vec![];
        let mut seed_to_soil: Mappings = Mappings::new();
        let mut soil_to_fertilizer: Mappings = Mappings::new();
        let mut fertilizer_to_water: Mappings = Mappings::new();
        let mut water_to_light: Mappings = Mappings::new();
        let mut light_to_temperature: Mappings = Mappings::new();
        let mut temperature_to_humidity: Mappings = Mappings::new();
        let mut temperature_to_humidity: Mappings = Mappings::new();
        let mut humidity_to_location: Mappings = Mappings::new();

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
            let mut val = get_dest(seed, &seed_to_soil);
            val = get_dest(val, &soil_to_fertilizer);
            val = get_dest(val, &fertilizer_to_water);
            val = get_dest(val, &water_to_light);
            val = get_dest(val, &light_to_temperature);
            val = get_dest(val, &temperature_to_humidity);
            val = get_dest(val, &humidity_to_location);
            println!("seed {} -> {}", seed, val);
            location = min(location, val);
        }
        println!("star1: {}", location);
    }
}
