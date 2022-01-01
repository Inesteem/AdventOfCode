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

fn get_idx(city : &str, map : &mut HashMap<String, usize>, i : &mut usize) -> usize {
    
    if !map.contains_key(city) {
        map.insert(city.to_string(), *i);
        *i += 1;
    }
    map[city]
}

#[derive(Debug, Clone)]
struct Edge {
    dst : usize,
    cost : usize,
}

struct Graph {
     edges : Vec<Vec<Edge>>,
}

impl Graph {
    fn new(size : usize) -> Self {
        Graph{edges : vec![vec![];size]}
    }

    fn add(&mut self, c1 : usize, c2 : usize, cost : usize) {
        self.edges[c1].push(Edge{ dst : c2, cost : cost});
        self.edges[c2].push(Edge{ dst : c1, cost : cost});
    }

    fn dijkstra(&self, curr_city : usize, num_cities : usize, num_visited : usize, visited : &mut Vec<bool>, cmp_func : &dyn Fn(usize, usize) -> usize, cmp_dist : usize) -> usize {
        if num_cities == num_visited { return 0; }
        let mut dist = cmp_dist;

        for edge in &self.edges[curr_city] {
            if visited[edge.dst] { continue; }
            visited[edge.dst] = true;
            dist = cmp_func(dist, edge.cost + self.dijkstra(edge.dst, num_cities, num_visited+1, visited, cmp_func, cmp_dist));
            visited[edge.dst] = false;
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

        let lines : Vec<&str>= input.lines().collect();
        let mut idx_map = HashMap::new();
        let mut idx = 0;

        let mut graph = Graph::new(lines.len() * 2);
        for line in &lines {
            let parts : Vec<&str> = line.split_whitespace().collect();
            let c1 = get_idx(parts[0], &mut idx_map, &mut idx);
            let c2 = get_idx(parts[2], &mut idx_map, &mut idx);

            graph.add(c1, c2, parts[4].parse().unwrap());
        }

        let mut visited = vec![false; idx];

        //209 too high
        let mut shortest_dist = 100000;
        let mut longest_dist =  0;
        for i in 0..idx {
            visited[i] = true;
            shortest_dist = min(shortest_dist, graph.dijkstra(i, idx, 1, &mut visited, &min, 10000));
            longest_dist = max(longest_dist, graph.dijkstra(i, idx, 1, &mut visited, &max, 0));
            visited[i] = false;
        }
        println!("star1 {}", shortest_dist);
        println!("star2 {}", longest_dist);
   }

}
