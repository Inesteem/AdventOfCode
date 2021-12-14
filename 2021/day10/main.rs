use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}


fn get_opening(c : char) -> char {
    match c {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        '>' => '<',
        _ => 'x'
    }
}

fn get_closing(c : char) -> char {
    match c {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => 'x'
    }
}

fn get_points_star1(c : char) -> u64{
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}


fn get_points_star2(c : char) -> u64{
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    }
}

fn main() {

    let input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>
            input = inputs,
        Err(_) => process::exit(0),
    }

    let lines : Vec<&str> = input.lines().collect();


    let mut points_star1 : u64 = 0;

    let mut scores_star2 : Vec<u64> = vec![];

    'outer: for line in lines {
        let mut points_star2 : u64 = 0;
        let mut openings : Vec<char> = vec![];

         for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<'=>  openings.push(c),
                ']' | ')' | '}' | '>'=> {
                    if openings.len() == 0 {
                        continue 'outer;
                    }

                    let last_char = openings[openings.len()-1];

                    if last_char != get_opening(c) {
                        //println!("Expected {} but found {} instead.", get_closing(last_char), c );
                        points_star1 += get_points_star1(c);
                        continue 'outer;
                    }

                    else {
                        openings.remove(openings.len()-1);
                    }
                },
                _ => process::exit(0),
            }
        }

        for i in (0..openings.len()).rev() {
            print!("{}", get_closing(openings[i]));
            points_star2 *= 5;
            points_star2 += get_points_star2(openings[i]);
        }
        println!(" : {}", points_star2);
        scores_star2.push(points_star2);

    }

    println!("star1 points: {}", points_star1);
    scores_star2.sort();
    println!("star2 points: {}",scores_star2[scores_star2.len()/2 as usize]);


}
