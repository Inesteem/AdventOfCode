use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Orbit{
    center: String,
    orbant: String
}

#[derive(Debug)]
struct Node<'a> {
    children:  Vec<&'a Node<'a>>,
    name: String,
}

impl <'a> Node<'a> {

    pub fn new(name : String) -> Node<'a> {
        Node {
            children: Vec::new(),
            name: name 
        }
    }

    pub fn adopt(&mut self, child_n : &'a Node<'a>) {
        self.children.push(child_n);
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    
    }

    pub fn print(&self, level : String) {
        print!("{}{} -[ ", level, self.name);
        for c in &self.children {
            print!("{} ", c.name);
        }
        println!("]");
        for c in &self.children {
            c.print([" ", &level].concat());
        }
    }

    pub fn cnt_refs(&self, level : u32, num : &mut u32){
        *num += level;
        for c in &self.children {
            c.cnt_refs(level+1, num);
        }

    }

    
}


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    println!("{}", &contents);
    Ok(contents)
}



fn main() {


    let mut tree_connections : Vec<(String,String)>;
    let mut tmp : Vec<String>;
    match read_inputs("../../../data/test.txt".to_string()) {
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
        if !nodes.contains_key(&orb.center) {
            nodes.insert(orb.center.clone(), Node::new(orb.center.clone())); 
        }
        if !nodes.contains_key(&orb.orbant) {
            nodes.insert(orb.orbant.clone(), Node::new(orb.orbant.clone())); 
        }

        unsafe {
            assert_ne!(orb.center, orb.orbant, "`a` ({:?}) must not equal `b` ({:?})", orb.center, orb.orbant);
            let mut parent : *mut Node =nodes.get_mut(&orb.center).unwrap() as *mut _;
            let mut child : *mut Node = nodes.get_mut(&orb.orbant).unwrap() as *mut _;
            (*parent).adopt(& mut (*child));
        }
    }

    let root = nodes.get(&"COM".to_string()).unwrap();
    root.print("".to_string());
    root.print("".to_string());

    let mut num : u32 = 0;
    root.cnt_refs(0, &mut num);
    println!("{}", num);
}
