use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::isize;
use std::collections::{HashSet, HashMap};

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
//    let files = vec!["test", "data"];
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
        for (c, dst) in &replacements {
            if map.contains_key(c) {
                map.get_mut(c).unwrap().push(dst);
            } else {
                map.insert(*c, vec![dst]);
            }
        }

        let mut set : HashSet<String> = HashSet::new();
        
        permut(&map, &mut set, &goal, 0);
        println!("{}", set.len());
        //770 to high
        //
        let mut checked = HashSet::new();
        let mut todo = HashSet::new();
        todo.insert("e".to_string());

        while !checked.contains(&goal.to_string())  {
            if todo.len() == 0 {break;}
            let mut next_todo = HashSet::new();

            for word in &todo {
                println!("{}", word);
                if word.len() > goal.len() { continue; }
                if checked.contains(word) { continue; }
                checked.insert(word.clone());
                permut(&map, &mut next_todo, word, 0);
            }

            todo = next_todo;
        }
    }
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
