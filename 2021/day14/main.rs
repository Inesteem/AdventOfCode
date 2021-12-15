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

fn star1_old(lines : &Vec<Vec<char>>, map : &HashMap<[char;2], char>) {

    let mut polymere = replace_polymere(&lines[0], &map);

    for i in 0..39 {
        polymere = replace_polymere(&polymere, &map);

        let mut counts : Vec<usize> = vec![];
        for c in &['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'] {
            let num = polymere.iter().filter(|&n| *n == *c).count();
            if num == 0 {
                continue;
            }

            print!("{} ", num);
            counts.push(num);
        }
        counts.sort();
        println!("");
        //println!("{}", counts[counts.len()-1] - counts[0]);
    }
}

#[derive(Debug)]
struct Info {
   c : char,
   i : usize,
}



fn cnt_patterns(map : &HashMap<[char;2], Info>,  poly : &Vec<char>) -> Vec<usize> {

    let mut patternCnt: Vec<usize> = vec![0; map.len()];
    for i in 1..poly.len() {
        let pattern : [char;2] = [poly[i-1], poly[i]];
        if map.contains_key(&pattern) {
            let idx = map.get(&pattern).unwrap().i;
            patternCnt[idx] += 1;
        }
    }
    patternCnt
}

fn get_num(c : char) -> usize {
     c as usize - 65
}

fn get_num_chars(map : &HashMap<[char;2], Info>, patternCnt : &Vec<usize>) -> Vec<usize> {
    let mut cntChars = vec![0;27];

    for (key, value) in map {
        let num = patternCnt[value.i as usize];

        cntChars[get_num(value.c)] += num;
    }

    cntChars
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


        let mut map : HashMap<[char;2], Info> = HashMap::new();
        let mut trafoMap : HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 2..lines.len() {
            let repl : [char;2] = [lines[i][0],lines[i][1]];
            let dst : char = lines[i][6];

            map.insert( repl, Info{ c : dst, i : i-2} );
        }

        //current pattern -> future pattern
        for (key, value) in &map {

            let mut trafo : Vec<usize> = vec![];
            for pattern in &[[key[0], value.c], [value.c, key[1]]] 
            {
                if map.contains_key(&*pattern) {
                    let idx = map.get(&*pattern).unwrap().i;
                    trafo.push(idx);
                }
            }
            trafoMap.insert( value.i, trafo ); 
        }

        let mut currPatternCnt = cnt_patterns(&map,  &lines[0]); 
        let mut overallPatternCnt = cnt_patterns(&map,  &lines[0]); 

        //for i in 0..9 { star1
        for i in 0..39 {
            let mut nextCurrPatternCnt : Vec<usize> = vec![0; map.len()];
            for pos in 0..currPatternCnt.len() {
                let num = currPatternCnt[pos];

                for pattern in &trafoMap[&pos] {
                    overallPatternCnt[*pattern] += num;
                    nextCurrPatternCnt[*pattern] += num;
                }

            }

//            println!("{:?}", overallPatternCnt);
            currPatternCnt = nextCurrPatternCnt;
        }

        let mut cntChars = get_num_chars(&map, &overallPatternCnt );

        //add start line characters
        for c in &lines[0] {
            cntChars[get_num(*c)] += 1;
        }

        //println!("{:?}",trafoMap);
       cntChars.sort();
//       println!("{:?}", cntChars);

        for i in 0..cntChars.len() {
            if cntChars[i] != 0 {
                println!("{}", *cntChars.last().unwrap() - cntChars[i]);
                break;
            }
        }

    }
}
