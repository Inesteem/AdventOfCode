extern crate lib;
use lib::io::*;
use std::process::exit;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Link{
    pub num: u32,
   // pub node: &'a Node<'a>,
    pub node: String,
}

impl Link {
    
    pub fn new(num : u32, node : &str) -> Link{
        Link {
            num : num,
            node : node.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub name : String,
    pub links : Vec<Link>,
}

impl Node {
    
    pub fn new(s : & str) -> Node{
        Node {
            name : s.to_string(),
            links : Vec::<Link>::new(),

        }
    }

    pub fn search(&self,needle:&str,bags : & HashMap<String,Node>)->bool {
        if self.name.eq(needle) { return true; }
        for l in &self.links {
           let node = bags.get(&l.node).unwrap();
           if node.search(needle,bags){ return true;}
        }
        false
    }
}




fn parse_line(line: & str) -> (String, u32){
   let splitted : Vec<&str>= line.split_whitespace().collect();
   let mut conc;
   let mut num : u32 = 0;
   if splitted.len() == 3 {
      conc = String::from(splitted[0]);
      conc.push_str(splitted[1]);
   } else {
      assert!(splitted.len() == 4);
      conc = String::from(splitted[1]);
      conc.push_str(splitted[2]);
      num = splitted[0].parse().expect("not a number");
   }
   (conc,num)
}


fn add_node(bag : & str, bags : & mut HashMap<String,Node>){
    //*table.entry(key).or_insert(0)
   if !bags.contains_key(bag) {
        println!("insert: {}",&bag);
        bags.insert(bag.to_string(),Node::new(&bag));
   }
}

fn main (){
    let mut bags = HashMap::new();
    loop {
        println!("");
        let line;
        match read_line_from_stdin() {
            Ok(l)  => line=l,
            Err(_) => {println!("error while reading from stdin"); exit(-1); },
        };
        if line.len() == 0 {break;} 
        let split = line[..line.len()-1].split(" contain");
        
        let parts = split.collect::<Vec<&str>>();
        

        let (name,num) = parse_line(parts[0]);
        add_node(&name,& mut bags);
        println!("{:?}\n   -> {} {}", &parts,&name,&num);

        if parts[1].eq(" no other bags.") {
           continue;
        }

        let subparts : Vec<&str> = parts[1].split(',').collect();
        for i in 0..subparts.len() {
            let (sname,num) = parse_line(&subparts[i]);
            add_node(&sname,& mut bags);
            println!("{:?}\n   -> {} {}", &subparts,&sname,&num);
            //let link_node = bags.get(&sname).unwrap();
            let node = bags.get_mut(&name).unwrap();
            node.links.push(Link::new(num,&sname));
        }
    }
    let mut num = 0;
    for key in bags.keys() { 
        //println!("{} -> {:?}", key, bags.get(key).unwrap()); 
        if bags.get(key).unwrap().search("shinygold",&bags) { 
            num += 1; 
            println!("{} contains a gold bag", key);
        }        
    }
    println!("bags that can contain a gold bag (without the gold bag itself) {}",num-1);
}
