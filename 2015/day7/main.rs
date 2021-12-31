use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_idx(s : &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

fn get_number(s : &str) -> u16 {
    s.parse().unwrap()
}

//fn get_value(regs : &HashMap<String, u16>, s : &str) -> u16 {
//    if regs.contains_key(s){
//        return regs[s];
//    }
//    match s.parse::<u16>() {
//        Ok(x) => x,
//        Err(_) => 0,
//    }
//}

fn parse(regs : &mut HashMap<String, u16>, parts_map : &HashMap<String, Vec<&str>>, reg : &str) -> u16 {

    match reg.parse::<u16>() {
        Ok(x) =>  return x,
        Err(_) => {},
    }

    let parts = &parts_map[reg];

    let val = match parts[1] {
        "->" => get_value(regs, parts_map, parts[0]),
        "AND" => get_value(regs, parts_map, parts[0]) & get_value(regs, parts_map, parts[2]),
        "OR" => get_value(regs, parts_map, parts[0]) | get_value(regs, parts_map, parts[2]),
        "LSHIFT" => get_value(regs, parts_map, parts[0]) << get_number(parts[2]),
        "RSHIFT" => get_value(regs, parts_map, parts[0]) >> get_number(parts[2]),
        _ => match parts[0] {

            "NOT" => !get_value(regs, parts_map, parts[1]),
            _=> panic!("unknown instruction: {}", parts[1]),
        },
    };
    regs.insert(reg.to_string(), val);
    val
}

fn get_value(regs : &mut HashMap<String, u16>, parts_map : &HashMap<String, Vec<&str>>, reg : &str) -> u16 {

    if regs.contains_key(reg) { return regs[reg]; }
    return parse(regs, parts_map, reg) ;
}
fn main() {
    let files = vec!["data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let lines : Vec<&str>= input.lines().collect();

        let mut parts_map = HashMap::new();

        for line in lines {
            let parts : Vec<&str> = line.split_whitespace().collect();
            let reg = parts[parts.len() - 1].to_string();

            if parts_map.contains_key(&reg) { panic!("something is wrong"); }

            parts_map.insert(reg, parts);
        }

        let mut regs = HashMap::new();
        let a1 = get_value(&mut regs, &mut parts_map, "a");
        println!("star1: {}", a1);


        regs = HashMap::new();
        regs.insert("b".to_string(), a1);
        let a2 = get_value(&mut regs, &mut parts_map, "a");
        println!("star2: {}", a2);

   }

}
