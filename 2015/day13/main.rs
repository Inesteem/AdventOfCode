use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::{min,max};

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_idx(guest: &str, map : &mut HashMap<String, usize>, i : &mut usize) -> usize {
    
    if !map.contains_key(guest) {
        map.insert(guest.to_string(), *i);
        println!("adding {} as {}", guest,i); 
        *i += 1;
    }
    map[guest]
}

struct Graph {
     edges : Vec<HashMap<usize, isize>>,
}

impl Graph {
    fn new(size : usize) -> Self {
        Graph{edges : vec![HashMap::new();size]}
    }

    fn add(&mut self, c1 : usize, c2 : usize, cost : isize) {
        self.edges[c1].insert(c2, cost);
    }

    fn dijkstra(&self, curr_guest : usize, goal_guest: usize, num_visited : usize, visited : &mut Vec<bool>) -> isize {
        if num_visited > 0 && curr_guest == goal_guest {
            let num_guests = visited.len();
            if num_visited == num_guests {return 0; }
            return -10000;
        }
        let mut dist = -1000;

        for (dst,cost) in &self.edges[curr_guest] {
            if visited[*dst] { continue; }
            visited[*dst] = true;
            let cost_rev = self.edges[*dst][&curr_guest];
            dist = max(dist, *cost + cost_rev + self.dijkstra(*dst, goal_guest, num_visited + 1, visited));
            visited[*dst] = false;
        }

        dist
    }
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

        let mut guest_map = HashMap::new();
        //set to zero for star1
        let mut idx = 1;

        let mut graph = Graph::new(lines.len() + 1);
        for line in &lines {
            let g1 = get_idx(line[0], &mut guest_map, &mut idx);
            let g2 = get_idx(&line[10][0..line[10].len()-1], &mut guest_map, &mut idx);

            let mut happiness : isize = line[3].parse().unwrap();
            if line[2] == "lose" { happiness *= -1; }
            graph.add(g1, g2, happiness);
        }

        //comment for star1
        for i in 1..idx {
            graph.add(0, i, 0);
            graph.add(i, 0, 0);

        }

        let mut visited = vec![false; idx];
        let mut happiness = graph.dijkstra(0, 0, 0,  &mut visited);
        println!("star1 {}", happiness);
   }

}
