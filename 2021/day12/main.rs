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

fn get_key<'a>(node : &'a str, map : &mut HashMap<&'a str, usize>, nextKey : usize) -> usize 
{
    *map.entry(node).or_insert(nextKey)
}

fn process_edge<'a>(node : &'a str, map : &mut HashMap<&'a str, usize>, power : &mut Vec<usize>, graph : &mut Vec<Vec<usize>>) -> usize 
{
    let len = map.len();
    let key = get_key(node, map, len );

    if key == len  {
        if node.chars().nth(0).unwrap().is_uppercase() {
            power.push(100000);
        } else {
            power.push(1);
        }

        graph.push(vec![]);
    }

    key
}

fn getWays(power : &mut Vec<usize>, graph : & Vec<Vec<usize>>, pos : usize, goal : usize, magic: usize) -> usize
{
    if(pos == goal) {
        if magic == graph.len() { return 1; }
        if power[magic] == 0 { return 1; }
        return 0;
    }

    power[pos] -= 1;

    let mut ways = 0;
    for next in &graph[pos] {
        if power[*next] == 0 { continue; }
        ways += getWays(power, graph, *next, goal, magic);
    }

    power[pos] += 1;


    ways
}

fn main() {

    let files = vec!["test1","test2","test3","data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<&str> = input.lines().collect();

        let mut map : HashMap<&str, usize> = HashMap::new();
        let mut graph : Vec<Vec<usize>> = vec![vec![]];
        let mut power : Vec<usize> = vec![];


        for line in &lines {
            let edge : Vec<&str> = line.split("-").collect(); 
            let keyA = process_edge(edge[0], &mut map, &mut power, &mut graph);
            let keyB = process_edge(edge[1], &mut map, &mut power, &mut graph);

            graph[keyA ].push(keyB);
            graph[keyB ].push(keyA);

        }
        let startIdx = *map.get("start").unwrap();
        let endIdx = *map.get("end").unwrap();
        let mut ways = getWays(&mut power, &graph, startIdx, endIdx, graph.len());
        for idx in 0..power.len() {
            if idx == startIdx || idx == endIdx { continue; }
            if power[idx] > 1 {continue;}
            power[idx] += 1;
            ways += getWays(&mut power, &graph, startIdx, endIdx, idx);
            power[idx] -= 1;
        }
        println!("ways: {}", ways);
    }
}
