use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::collections::HashMap;
//omg, this is fucking horrible code
//I am sorry! But, I am to drunken for rust right now
//why do I need those clones, wtf is happening there

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn sharedChars(a : &str, b : &str) -> u32 {

    let mut num : u32 = 0;
    for c1 in a.chars() {
        for c2 in b.chars() {
            if c1 == c2 {
                num += 1;
                break;
            }
        }
    }
    num
}

// 1 4 7 : 9 2
// 2 : 5 6
// 6 : 3 0
//1 4 7 8
//     x     x     x x
//   0 1 2 3 4 5 6 7 8 9
//   6 2 5 5 4 5 6 3 7 6
//
//a  *   * *   * * * * *
//b  *       * * *   * *
//c  * * * * *     * * *
//d      * * * * *   * *
//e  *   *       *   *
//f  * *   * * * * * * *
//g  *   * *   * *   * *


//eindeutig 1
fn isNine(s : &str, map : &HashMap<u32, String>) -> bool {
    if !map.contains_key(&1) || !map.contains_key(&4) || !map.contains_key(&7) {
        return false;
    }

    if !(sharedChars(s, &map[&1]) == 2) { return false; }

    if !(sharedChars(s, &map[&4]) == 4) { return false; }

    if !(sharedChars(s, &map[&7]) == 3) { return false; }

    return true;
}

//eindeutig 1
fn isTwo(s : &str, map : &HashMap<u32, String>) -> bool {
    if !map.contains_key(&1) || !map.contains_key(&4) || !map.contains_key(&7) {
        return false;
    }

    if !(sharedChars(s, &map[&1]) == 1) { return false; }

    if !(sharedChars(s, &map[&4]) == 2) { return false; }

    if !(sharedChars(s, &map[&7]) == 2) { return false; }

    return true;
}

//needs 2
fn isSix(s : &str, map : &HashMap<u32, String>) -> bool {
    if !map.contains_key(&2) {
        return false;
    }

    if !(sharedChars(s, &map[&1]) == 1) { return false; }

    if !(sharedChars(s, &map[&4]) == 3) { return false; }

    if !(sharedChars(s, &map[&7]) == 2) { return false; }

    if !(sharedChars(s, &map[&2]) == 4) { return false; }

    return true;
}


//needs 2
fn isFive(s : &str, map : &HashMap<u32, String>) -> bool {
    if !map.contains_key(&2) {
        return false;
    }

    if !(sharedChars(s, &map[&1]) == 1) { return false; }

    if !(sharedChars(s, &map[&4]) == 3) { return false; }

    if !(sharedChars(s, &map[&7]) == 2) { return false; }

    if !(sharedChars(s, &map[&2]) == 3) { return false; }

    return true;
}

//needs 6
fn isThree(s : &str, map : &HashMap<u32, String>) -> bool {
    if !map.contains_key(&6) {
        return false;
    }

    if !(sharedChars(s, &map[&1]) == 2) { return false; }

    if !(sharedChars(s, &map[&4]) == 3) { return false; }

    if !(sharedChars(s, &map[&7]) == 3) { return false; }

    if !(sharedChars(s, &map[&6]) == 4) { return false; }

    return true;
}

//needs 6
fn isZero(s : &str, map : &HashMap<u32, String>) -> bool {
    if !map.contains_key(&6) {
        return false;
    }


    if !(sharedChars(s, &map[&1]) == 2) { return false; }

    if !(sharedChars(s, &map[&4]) == 3) { return false; }

    if !(sharedChars(s, &map[&7]) == 3) { return false; }

    if !(sharedChars(s, &map[&6]) == 5) { return false; }

    return true;
}



fn main() {

    let mut input: String;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            input = inputs,
        Err(_) => process::exit(0),
    }

    let lines : Vec<&str> = input.lines().collect();


    let mut sum : u32= 0;
    for line in &lines {

        let l : Vec<&str> = line.split(" | ").collect();

        let mut parts1 : Vec<&str> = l[0].split_whitespace().collect();
        let mut parts2 : Vec<&str> = l[1].split_whitespace().collect();
        

        parts1.append(&mut parts2);
        
        let mut parts : Vec<String> = vec![];
        for i in 0..parts1.len() {
            let mut l: Vec<char> = parts1[i].chars().collect();
            l.sort_unstable();
            let j : String = l.into_iter().collect();
            parts.push(j);
        }
    
        let mut found = 0;
        let mut intMap = HashMap::new();
        let mut strMap = HashMap::new();
        while found != 10 {
            for p in &parts {
                let part = (*p).clone();
                if strMap.contains_key(&part) {
                    continue;
                }
                let len = part.len();
                if len == 2 && !intMap.contains_key(&1) {
                    intMap.insert(1, part.clone());
                    strMap.insert(part.clone(), 1);
                    found += 1;
                } else if len == 3 && !intMap.contains_key(&7) {
                    intMap.insert(7, part.clone());
                    strMap.insert(part.clone(), 7);
                    found += 1;
                } else if len == 4 && !intMap.contains_key(&4) {
                    intMap.insert(4, part.clone());
                    strMap.insert(part.clone(), 4);
                    found += 1;
                } else if len == 7 && !intMap.contains_key(&8) {
                    intMap.insert(8, part.clone());
                    strMap.insert(part.clone(), 8);
                    found += 1;
                } else if len == 5 { // 2 3 5
                    if !intMap.contains_key(&2) && isTwo(&part, &intMap) {
                        intMap.insert(2, part.clone());
                        strMap.insert(part.clone(), 2);
                        found += 1;
                        continue;
                    }
                    if !intMap.contains_key(&3) && isThree(&part, &intMap) {
                        intMap.insert(3, part.clone());
                        strMap.insert(part.clone(), 3);
                        found += 1;
                        continue;
                    }
                    if !intMap.contains_key(&5) && isFive(&part, &intMap) {
                        intMap.insert(5, part.clone());
                        strMap.insert(part.clone(), 5);
                        found += 1;
                        continue;
                    }
                } else if len == 6 { // 0 6 9
                    if !intMap.contains_key(&0) && isZero(&part, &intMap) {
                        intMap.insert(0, part.clone());
                        strMap.insert(part.clone(), 0);
                        found += 1;
                        continue;
                    }
                    if !intMap.contains_key(&6) && isSix(&part, &intMap) {
                        intMap.insert(6, part.clone());
                        strMap.insert(part.clone(), 6);
                        found += 1;
                        continue;
                    }
                    if !intMap.contains_key(&9) && isNine(&part, &intMap) {
                        intMap.insert(9, part.clone());
                        strMap.insert(part.clone(), 9);
                        found += 1;
                        continue;
                    }

                } 
            }
        }

        parts2  = l[1].split_whitespace().collect();
 
        let mut parts3 : Vec<String> = vec![];
        for i in 0..parts2.len() {
            let mut l: Vec<char> = parts2[i].chars().collect();
            l.sort_unstable();
            let j : String = l.into_iter().collect();
            parts3.push(j);
        }
    

        let mut num = 0;
        for p in &parts3 {
           num *= 10;
           num += strMap[p];
        }
        println!("{}", num);
        sum += num;
    }
    println!("{}", sum);

    ////star1
    //let mut num : u32= 0;
    //for line in lines {

    //    let l : Vec<&str> = line.split(" | ").collect();
    //    let parts : Vec<&str> = l[1].split_whitespace().collect();

    //    for part in parts {
    //        let len = part.len();
    //        if len == 2 || len == 4 || len == 3 || len == 7 {
    //            num += 1;
    //        }
    //    }
    //}

    //println!("{}", num);



}
