use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use std::cmp::min;
use std::collections::{HashSet, HashMap};
//Disclaimer: I am not proud of this ugly piece of shit code
//I got frustrated over star2 and wanted to finish it off as soon as possible.
//In the internet I found some pretty smart solutions, since the input
//is some sort of context free grammar. Very cool, so there are much
//smarter solutions out there.
const DEBUG : bool = false;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn main() {
    //    std::io::stdin().read_to_string(&mut input).unwrap();
    let files = vec!["data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }
        let lines : Vec<&str> = input.lines().collect();

        let goal: &str= lines[lines.len()-1];
        let replacements :Vec<(&str, &str)> = lines[0..lines.len()-2].iter().map(|w|
                                                                                 {
                                                                                     let parts : Vec<&str> = w.split_whitespace().collect();
                                                                                     return (parts[0], parts [2]) 
                                                                                 }).collect();

        let mut map : HashMap<&str, Vec<&str>> = HashMap::new();
        let mut rev_map : HashMap<&str, Vec<&str>> = HashMap::new();
        let mut rev_map = vec![];
        let mut rev_map_opt = vec![];
        for (c, dst) in &replacements {
            if map.contains_key(c) {
                map.get_mut(c).unwrap().push(dst);
            } else {
                map.insert(*c, vec![dst]);
            }
            if dst.contains("Ar") {
                rev_map.push((*dst, *c));
            } else {
                rev_map_opt.push((*dst, *c));
            }

        }
        rev_map.sort_by(|a,b| b.0.len().partial_cmp(&a.0.len()).unwrap());
        let mut set : HashSet<String> = HashSet::new();

        permut(&map, &mut set, &goal, 0);
        println!("star1: {}", set.len());

        let mut min_steps : usize = 999999999999999999;
        let mut checked = HashMap::new();
        steps(&rev_map_opt, &rev_map, &goal, &mut checked, 0, &mut min_steps);
        println!("{}", min_steps);
    }
}

fn steps(map : &Vec<(&str, &str)>, map2 : &Vec<(&str, &str)>, w : &str, checked : &mut HashMap<String, usize>, curr_steps : usize, min_steps : &mut usize) -> usize {
    if curr_steps >= *min_steps { return 9999999999; }
    //(I use the assumption (which is proven somewhere), that there is only one possible way to transform e into the given sequence. I found this in the internet - again I wasnt smart enough for this one
    if w == "e" { println!("star2: {}", curr_steps); process::exit(0);}//*min_steps = curr_steps; return 0; }

    let mut word : String = w.to_string();
    let mut pos = 0;
    let mut cnt = 0;
    'outer: loop {
        if pos >= word.len() { break; }
        for (pat,dst) in &*map2 {
            if pat.len() > word.len() - pos { continue; }
            if *pat == &word[pos..pos+pat.len()] {
                word =  word[0..pos].to_string() + dst + &word[pos+pat.len()..word.len()].to_string();
                pos += pat.len();
                cnt += 1;
                continue 'outer;

            }
        }
        pos += 1;
    }
    if cnt > 0 {
        return steps(map, map2, &word, checked, curr_steps+cnt, min_steps);
    }

    let mut curr_min_steps = 9999999999;
    let mut next_round = vec![];
    for pos in 0..word.len() {
        for (pat,dst) in &*map {
            if pat.len() > word.len() - pos { continue; }
            if *pat == &word[pos..pos+pat.len()] {
                if word.len() > 3 && dst == &"e" {continue;}
                let next_ : String = word[0..pos].to_string() + dst + &word[pos+pat.len()..word.len()].to_string();
                if checked.contains_key(&next_) {
                    curr_min_steps = min(curr_min_steps, *checked.get(&next_).unwrap());
                    continue;
                }
                next_round.push(next_);
            }
        }
    }
    for next_ in next_round {
        let num_steps = steps(map, map2, &next_, checked, curr_steps+1, min_steps);
        curr_min_steps = min(curr_min_steps, num_steps);
    }
    checked.insert(word.to_string(), 1+curr_min_steps);
    return 1+curr_min_steps;

}


fn addString(map : &HashMap<&str,Vec<&str>>, set : &mut HashSet<String>, word : &str, pos : usize, len : usize) {
    for repl in map.get(&word[pos..pos+len]).unwrap()
    {
        let w2 : String = word[0..pos].to_string() + repl + &word[pos+len..word.len()].to_string();
        set.insert(w2);
    }
}

fn permut(map : &HashMap<&str,Vec<&str>>, set : &mut HashSet<String>, word : &str, pos : usize) {

        if pos == word.len() {
            return;
        }
        if map.contains_key(&word[pos..pos+1]) 
        {
            addString(map, set, word, pos, 1);
        }
        if pos < word.len()-1 && map.contains_key(&word[pos..pos+2]) 
        {
            addString(map, set, word, pos, 2);
        }

        permut(map, set, word, pos+1);
}
