use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::usize;
use std::cmp::min;
use std::collections::HashMap;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

//fn diff(a : usize, b : usize) -> usize {
//    if a > b { return a - b; }
//    b - a
//}
#[derive(Debug, Clone)]
struct Package {
    w: usize,
    q: usize,
    n: usize,
}

impl Package {
    fn add_weight(&mut self, w : usize)
    {
        self.w += w;
        self.q *= w;
        self.n += 1;
    }
}

fn get_q(mut packages : &Vec<Package>) -> usize
{
    let mut q = packages[0].q;
    for i in 1..packages.len() {
        if packages[i].n != packages[i-1].n {
            break;
        }
        q = min(q, packages[i].q);
    }
    return q;
}


fn get_key(pos : usize, mut packages: &Vec<Package>) -> u128
{
    let mut key = pos as u128;
    for i in 0..packages.len() {
        key <<= 12;
        key |= packages[i].w as u128;
    }
    for i in 0..packages.len() {
        assert!(packages[i].n < 32);
        key <<= 5;
        key |= packages[i].n as u128;
    }
    key
}
fn all_the_same(packages : &Vec<Package>) -> bool {
    for i in 1..packages.len() {
        if packages[i].w != packages[0].w { return false; }
    }
    true
}
fn abs(a : usize, b : usize) -> usize {
    if a > b { return a-b;}
    b-a
}

fn get_diff(mut packages : Vec<Package>) -> usize {

    packages.sort_by(|a,b| b.w.partial_cmp(&a.w).unwrap());
    let mut needed = 0;

    for i in 1..packages.len() {
        needed += abs(packages[i].w, packages[0].w);
    }

    needed
}

fn good_func_name(
    weights : &Vec<usize>,
    sum : usize,
    pos : usize,
    mut packages : Vec<Package>,
    min_w : &mut usize,
    min_q : &mut usize,
    min_n : &mut usize,
    dp : &mut HashMap<u128, usize>)
{

    packages.sort_by(|a,b| a.n.partial_cmp(&b.n).unwrap());
    let q = get_q(&packages);
    let n = packages[0].n;
    if n > *min_n { return;}

    let key = get_key(pos, &packages);
    if dp.contains_key(&key) && *dp.get(&key).unwrap() <= q { return;}
    dp.insert(key, q);

    if pos >= weights.len()
    {
        if all_the_same(&packages)
        {
            if n <= *min_n
            {
                *min_n = n;
                *min_w = packages[0].w;
                *min_q = min(*min_q, q);
            }
        }
        return;
    }
    let needed = get_diff(packages.clone());
    if sum < needed { return; }

    let w = weights[pos];
    for i in 0..packages.len() {
        let mut p = packages.clone();
        p[i].add_weight(w);
        good_func_name(weights, sum-w, pos+1, p, min_w, min_q, min_n, dp);
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

        let mut weights: Vec<usize>= input.lines().
            map(|line| line.parse().unwrap()).
            collect();
        weights.reverse();
        let sum: usize = weights.iter().sum();
        let mut min_w = usize::MAX;
        let mut min_q = usize::MAX;
        let mut min_n = usize::MAX;
        {
            let mut dp = HashMap::new();
            good_func_name(&weights, sum, 0,
                           vec![Package {w : 0, q : 1, n : 0},
                                Package {w : 0, q : 1, n : 0},
                                Package {w : 0, q : 1, n : 0}],
                           &mut min_w, &mut min_q, &mut min_n, &mut dp);
            println!("star1: w:{} q:{} n:{}", min_w, min_q, min_n);
        }
        {
            let mut dp = HashMap::new();
            good_func_name(&weights, sum, 0,
                           vec![Package {w : 0, q : 1, n : 0},
                                Package {w : 0, q : 1, n : 0},
                                Package {w : 0, q : 1, n : 0},
                                Package {w : 0, q : 1, n : 0}],
                           &mut min_w, &mut min_q, &mut min_n, &mut dp);
            println!("star2: w:{} q:{} n:{}", min_w, min_q, min_n);
        }
    }
}
