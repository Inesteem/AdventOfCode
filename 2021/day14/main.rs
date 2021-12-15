use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::collections::HashMap;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}


fn replace_polymere(poly : &Vec<char>,  map : &HashMap<[char;2], char>) -> Vec<char>{
    let mut newPoly : Vec<char> = vec![poly[0]];

    for i in 1..poly.len() {
        let pattern : [char;2] = [poly[i-1], poly[i]];
        if map.contains_key(&pattern) {
            newPoly.push(*map.get(&pattern).unwrap());
        }
        newPoly.push(poly[i]);
    }
    newPoly
}


fn main() {
    let width : usize = 1500;
    let height : usize= 1500;
    let files = vec!["test", "data"];

    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

        let mut map : HashMap<[char;2], char> = HashMap::new();

        for i in 2..lines.len() {
            let repl : [char;2] = [lines[i][0],lines[i][1]];
            let dst : char = lines[i][6];

            map.insert( repl, dst );
        }

        let mut polymere = replace_polymere(&lines[0], &map);

        for i in 0..39 {
            polymere = replace_polymere(&polymere, &map);

        let mut counts : Vec<usize> = vec![];
        for c in ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'] {
            let num = polymere.iter().filter(|&n| *n == c).count();
            if num == 0 {
                continue;
            }

            //println!("{}: {}", c, num);
            counts.push(num);
        }
        counts.sort();
        println!("{:?}", counts);
        //println!("{}", counts[counts.len()-1] - counts[0]);
        }
    }
}
