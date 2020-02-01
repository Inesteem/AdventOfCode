use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::collections::HashMap;
use std::cell::Cell;


#[derive(Debug)]
struct Orbit{
    center: String,
    orbant: String
}

struct Node<'a> {
    children:  Cell< Vec<&'a Node<'a>> >,
    name: String,
    dist : Cell<i32>,
}

impl <'a> Node<'a> {

    pub fn new(name : String) -> Node<'a> {
        Node {
            children: Cell::new(Vec::new()),
            name: name,
            dist : Cell::new(0),
        }
    }

    pub fn adopt(& self, child_n : &'a Node<'a>) {
        let mut v = self.children.take();
        v.push(child_n);
        self.children.set(v);
    }

    pub fn print(&self, level : String) {
        let v = self.children.take();
        print!("{}{} -[ ", level, self.name);
        for c in &v {
            print!("{} ", c.name);
        }
        println!("]");
        for c in &v {
            c.print([" ", &level].concat());
        }
        self.children.set(v);
    }

    pub fn cnt_refs(&self, level : u32, num : &mut u32){
        let v = self.children.take();
        *num += level;
        for c in &v {
            c.cnt_refs(level+1, num);
        }
        self.children.set(v);

    }

    pub fn calc_dist1(& self) -> i32{
        if self.name == "YOU" {
            return 0;
        }

        let v = self.children.take();
        for c in &v {
            let dist =  c.calc_dist1();
            if dist != -1 {
                self.children.set(v);
                self.dist.set(dist+1);
                return  dist + 1;
            }
        }
        self.children.set(v);
        return -1;
    }
    pub fn calc_dist2(& self, d : i32){

        if self.dist.get() == 0 {
            self.dist.set(d);
        }

        let v = self.children.take();

        for c in &v {
           c.calc_dist2(self.dist.get()+1); 
        }

        self.children.set(v);
    }
}


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
//    println!("{}", &contents);
    Ok(contents)
}



fn main() {


    let mut tmp : Vec<String>;
    match read_inputs("../../../data/day6.txt".to_string()) {
//    match read_inputs("../../../data/test.txt".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            tmp = inputs.split("\n")
            .map(|x| x.to_string())
            .collect(),
        Err(_) => process::exit(0),
            
    }

    let mut nodes = HashMap::new();

    for line in &tmp {
    
        let splitted : Vec<String>=  line.split(")").map(|x| x.to_string()).collect();
      
        let orb = Orbit{center : splitted[0].clone(), orbant : splitted[1].clone() };
        {
            if !nodes.contains_key(&orb.center) {
                nodes.insert(orb.center.clone(), Node::new(orb.center.clone())); 
            }
            if !nodes.contains_key(&orb.orbant) {
                nodes.insert(orb.orbant.clone(), Node::new(orb.orbant.clone())); 
            }
        }
    }
    for line in &tmp {
        let splitted : Vec<String>=  line.split(")").map(|x| x.to_string()).collect();
      
        let orb = Orbit{center : splitted[0].clone(), orbant : splitted[1].clone() };
        let parent =nodes.get(&orb.center).unwrap();
        let child = nodes.get(&orb.orbant).unwrap();
        (*parent).adopt(&(*child));
//        unsafe {
//            assert_ne!(orb.center, orb.orbant, "`a` ({:?}) must not equal `b` ({:?})", orb.center, orb.orbant);
//            let mut parent : *mut Node =nodes.get_mut(&orb.center).unwrap() as *mut _;
//            let mut child : *mut Node = nodes.get_mut(&orb.orbant).unwrap() as *mut _;
//            (*parent).adopt(& mut (*child));
//        }
    }

    let root = nodes.get(&"COM".to_string()).unwrap();
////    root.print("".to_string());
//
//    let mut num : u32 = 0;
//    root.cnt_refs(0, &mut num);
//    println!("{}", num);
//    
    root.calc_dist1();
    root.calc_dist2(0);

    let san = nodes.get(&"SAN".to_string()).unwrap();
    println!("{}", san.dist.get() - 2);
}

