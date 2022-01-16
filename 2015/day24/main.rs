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
    
    fn add_weight(&self, w : usize) -> Self 
    {
        Package{ w : self.w + w, q : self.q * w, n : self.n + 1}
    }

}

fn get_q(p1 : &Package, p2 : &Package, p3 : &Package) -> usize
{
    let mut v = vec![p1,p2,p3];
    v.sort_by(|a,b| a.n.partial_cmp(&b.n).unwrap());
    if v[0].n == v[1].n {
        if v[1].n == v[2].n { return min(min(v[0].q, v[1].q), v[2].q); }
        return min(v[0].q, v[1].q);
    }
    v[0].q
}

fn get_key(pos : usize, p1 : &Package, p2 : &Package, p3 : &Package) -> usize
{
    //pos 5 bit
    //w1 11 bit
    //w2 11 bit
    //w3 11 bit
    //-> 38 bit
    //n1 -> 4 bit
    //n2 -> 4 bit
    //n3 -> 4 bit
    //-> 60 bit
    //
//    let q = get_q(p1,p2,p3);
    let mut v = vec![p1,p2,p2];
    v.sort_by(|a,b| a.n.partial_cmp(&b.n).unwrap());
    let mut key = pos;
    key <<= 11;
    key |= p1.w;
    key <<= 11;
    key |= p2.w;
    key <<= 11;
    key |= p3.w;
    key <<= 4;
    key |= p1.n;
    key <<= 4;
    key |= p2.n;
    key <<= 4;
    key |= p3.n;
    key
}

fn abs(a : usize, b : usize) -> usize {
    if a > b { return a-b;}
    b-a
}

fn good_func_name(weights : &Vec<usize>, sum : usize, pos : usize, p1 : Package, p2 : Package, p3 : Package, min_w : &mut usize, min_q : &mut usize, min_n : &mut usize, dp : &mut HashMap<usize, usize>) {

    let q = get_q(&p1,&p2,&p3);
    let n = min(min(p1.n, p2.n), p3.n);
    if n > *min_n { return;}

    let key = get_key(pos, &p1, &p2, &p3);
    if dp.contains_key(&key) && *dp.get(&key).unwrap() <= q { return;}
    dp.insert(key, q);

    if pos >= weights.len()
    {
        if p1.w == p2.w && p2.w == p3.w
        {
            println!("{} {} {} {} {} {}", p1.w ,p2.w, p3.w, p1.q, p2.q, p3.q);
            println!("{} {} {} {}", *min_w, *min_q, p1.w, q);
            if n <= *min_n
            {
                *min_n = n;
                *min_w = p1.w;
                *min_q = min(*min_q, q);
            }
        }
        return;
    }

    if sum < abs(p1.w, p2.w) + abs(p1.w, p3.w) { return; }
    let w = weights[pos];

    good_func_name(weights, sum-w, pos + 1, p1.add_weight(w), p2.clone(), p3.clone(), min_w, min_q, min_n, dp);
    good_func_name(weights, sum-w, pos + 1, p1.clone(), p2.add_weight(w), p3.clone(), min_w, min_q, min_n, dp);
    good_func_name(weights, sum-w, pos + 1, p1, p2, p3.add_weight(w), min_w, min_q, min_n, dp);
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

        let mut weights: Vec<usize>= input.lines().
            map(|line| line.parse().unwrap()).
            collect();
        weights.reverse();
        let sum: usize = weights.iter().sum();
        let mut min_w = usize::MAX;
        let mut min_q = usize::MAX;
        let mut min_n = usize::MAX;

        let mut dp = HashMap::new();
        good_func_name(&weights, sum, 0,
                       Package {w : 0, q : 1, n : 0},
                       Package {w : 0, q : 1, n : 0},
                       Package {w : 0, q : 1, n : 0},
                       &mut min_w, &mut min_q, &mut min_n, &mut dp);
        println!("w:{} q:{} n:{}", min_w, min_q, min_n);
    }
}
