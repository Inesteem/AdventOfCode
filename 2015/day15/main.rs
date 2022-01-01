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

struct Ingredient {
    capacity : i128,
    durability : i128, 
    flavor : i128,
    texture : i128,
    calories : i128,
}

impl Ingredient {
    fn get(&self, idx : usize) -> i128 {
        [self.capacity, self.durability, self.flavor, self.texture, self.calories][idx]
    }
}

fn value(ingredients : &Vec<Ingredient>, nums: &Vec<i128>) -> i128 {
    let mut mul = 1;
    for a in 0..4 {
        let mut sum = 0;
        for i in 0..ingredients.len() {
            sum += ingredients[i].get(a) * nums[i];
        }
        mul *= max(0,sum);
    }
    mul
}

fn calories(ingredients : &Vec<Ingredient>, nums: &Vec<i128>) -> i128 {
    let mut sum = 0;
    for i in 0..ingredients.len() {
        sum += ingredients[i].get(4) * nums[i];
    }
    sum
}

fn get_best_score(ingredients : &Vec<Ingredient>, pos : usize, nums : &mut Vec<i128>, left : i128) -> i128 {
    if pos == ingredients.len() -1 {
        nums[pos] = left;
        //comment if star1
        if calories(ingredients, nums) != 500 {return -1000};
        return value(ingredients, nums);
    }

    let mut best_score = -1000;

    for i in 0..left {
        nums[pos] = i;
        best_score = max(best_score, get_best_score(ingredients, pos + 1, nums, left - i));
    }

    best_score
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

        let lines : Vec<Vec<&str>>= input.lines().map(|line| line.split_whitespace().collect()).collect();
        let mut ingredients  = vec![];
        for line in &lines {
            let capacity : i128 = line[2][0..line[2].len()-1].parse().unwrap();
            let durability : i128 = line[4][0..line[4].len()-1].parse().unwrap();
            let flavor : i128 = line[6][0..line[6].len()-1].parse().unwrap();
            let texture : i128 = line[8][0..line[8].len()-1].parse().unwrap();
            let calories : i128 = line[10].parse().unwrap();

            ingredients.push(Ingredient{capacity : capacity, durability : durability, flavor : flavor, texture : texture, calories : calories});

        }
        let mut nums = vec![0; ingredients.len()];
        println!("{}", get_best_score(&ingredients, 0, &mut nums, 100));

    }
}
