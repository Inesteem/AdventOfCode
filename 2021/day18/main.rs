use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::cmp::max;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}
fn read_in_one_char() {
    let input: Option<i32> = std::io::stdin()
        .bytes() 
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
}

type ChildNode<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    left : ChildNode<T>,
    right: ChildNode<T>,
    num: T,
    depth : usize,
    max : u8,
}

impl Node<u8> {
    pub fn new(line : &Vec<char>, pos : &mut usize) -> Self {
        let mut left  = None;
        let mut right = None;
        let mut num = 0;
        let mut depth = 0;
        let mut m = 0;

        loop {
            match line[*pos] {
                '[' => {
                    *pos += 1;
                    left = Some(Box::new(Node::<u8>::new(line, pos)));
                    right = Some(Box::new(Node::<u8>::new(line, pos)));
                    depth = max(left.as_ref().unwrap().depth, right.as_ref().unwrap().depth) + 1;
                    m = max(left.as_ref().unwrap().max, right.as_ref().unwrap().max);
                    break;
                },
                ']' => {*pos += 1;}
                ',' => {*pos += 1;}
                _ => {
                    num = line[*pos].to_digit(10).unwrap();
                    m = num as u8;
                    *pos += 1;
                    break;
                },
            }
        }

        Node::<u8>{left : left, right : right, num : num as u8, depth : depth, max : m}
    }


    pub fn print(&self) {
        if !self.left.is_none() {
            print!("[");
            self.left.as_ref().unwrap().print();
            print!(",");
            self.right.as_ref().unwrap().print();
            print!("]");
        } else {
            print!("{}", self.num);
        }
    }

    pub fn merge_left(&mut self, val : u8) {
        if !self.left.is_none() {
            self.left.as_mut().unwrap().merge_left(val);
        } else {
            self.num += val;
        }
        self.set_max();
    }

    pub fn merge_right(&mut self, val : u8) {
        if !self.right.is_none() {
            self.right.as_mut().unwrap().merge_right(val);
        } else {
            self.num += val;
        }
        self.set_max();
    }

    pub fn set_depth(&mut self) {
        if self.left.is_none(){
            self.depth = 0;
            return;
        }

        let d1 = self.left.as_ref().unwrap().depth;
        let d2 = self.right.as_ref().unwrap().depth;
        self.depth = max(d1,d2) + 1;

    }
    pub fn set_max(&mut self) {
        if self.left.is_none(){
            self.max = self.num;
            return;
        }

        let m1 = self.left.as_ref().unwrap().max;
        let m2 = self.right.as_ref().unwrap().max;
        self.max = max(m1, m2);

    }
    pub fn explode(&mut self, depth : isize) -> (bool, u8, u8) {
        if (self.depth as isize) < depth {return (false,0,0);}

        if depth == 1 {
            let ret = (true, self.left.as_ref().unwrap().num, self.right.as_ref().unwrap().num);
            self.left = None;
            self.right= None;
            self.num = 0;
            self.set_depth();
            self.set_max();
            return ret;
        }

        if !self.left.is_none() {
            let res = self.left.as_mut().unwrap().explode(depth-1);
            if res.0 {
                if res.2 != 0{
                    self.right.as_mut().unwrap().merge_left(res.2);
                    self.set_depth();
                    self.set_max();
                    return (res.0, res.1, 0);
                }
                self.set_depth();
                self.set_max();
                return res;
            }
        }

        if !self.right.is_none() {
            let res = self.right.as_mut().unwrap().explode(depth-1);
            if res.0 {
                if res.1 != 0 {
                    self.left.as_mut().unwrap().merge_right(res.1);
                    self.set_depth();
                    self.set_max();
                    return (res.0, 0, res.2);
                }
                self.set_depth();
                self.set_max();
                return res;
            }
        }

        return (false,0,0);

    }

    pub fn split(&mut self) -> bool {
        if self.max < 10 { return false;}

        if self.depth == 0 {
            let left_num = self.num / 2;
            let right_num = (self.num + 1) / 2;
            self.left = Some(Box::new(Node::<u8>{left : None, right : None, num : left_num, depth : 0, max : left_num}));
            self.right = Some(Box::new(Node::<u8>{left: None, right : None, num : right_num, depth : 0, max : right_num}));

            self.set_depth();
            self.set_max();
            return true;
        }

        if self.left.as_mut().unwrap().split() || self.right.as_mut().unwrap().split() {
            self.set_depth();
            self.set_max();
            return true;
        }

        false
    }


    pub fn magnitude(&self) -> usize {
        if self.depth == 0 {
            return self.num as usize;
        }

        let lm = self.left.as_ref().unwrap().magnitude();
        let rm = self.right.as_ref().unwrap().magnitude();

        3*lm + 2*rm
    }
}

#[derive(Debug, Clone)]
struct BinaryTree<T> {
    head : Option<Node<T>>
}

impl BinaryTree<u8> {
    //pub fn new(head: Node<u8>) -> Self {
    //    BinaryTree::<u8> {head : Some(head) }
    //}

    pub fn new(line : &str) -> Self {
        let v : Vec<char> = line.chars().collect();
        let mut pos = 0;

        let mut head = Node::<u8>::new(&v, &mut pos);
        BinaryTree::<u8> {head : Some(head) }
    }


    pub fn depth(&self) -> usize {
        if !self.head.is_none() {
            return self.head.as_ref().unwrap().depth;
        }
        return 0;
    }


    pub fn reduce(&mut self) {
        if !self.head.is_none() {
            let mut head : &mut Node<u8> = self.head.as_mut().unwrap();
            let mut check = true;
            while check {
                check = head.explode(5).0 || head.split();
            }
        }
    }

    pub fn add(left : BinaryTree<u8>, right : BinaryTree<u8>) -> Self {
        if left.head.is_none() && right.head.is_none() { }
        let mut head1 = left.head.unwrap();
        let mut head2 = right.head.unwrap();

        let depth = max(head1.depth, head2.depth) + 1;
        let max = max(head1.max, head2.max);
        BinaryTree::<u8>{
            head : Some(Node::<u8>{
                left :  Some(Box::new(head1)),
                right : Some(Box::new(head2)),
                num : 0,
                depth : depth,
                max : max})}
    }

    pub fn magnitude(&self) -> usize {
        if !self.head.is_none() {
            return self.head.as_ref().unwrap().magnitude();
        }
        return 0;
    }
    pub fn print(&self) {
        if !self.head.is_none() {
            self.head.as_ref().unwrap().print();
            println!();
        }
    }
}

fn main() {
    let files = vec!["homework", "data"];
    let star1 = false;
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<&str>= input.lines().collect();
        let mut trees = vec![];
        for line in lines {
            let mut tree = BinaryTree::<u8>::new(&line);
            tree.reduce();
            trees.push(tree);
        }
        if star1 {
            let mut tree = trees.remove(0);
            tree.print();
            tree.reduce();
            while trees.len() > 0 {
                tree = BinaryTree::<u8>::add(tree, trees.remove(0));
                tree.reduce();
            }
            println!("{}", tree.magnitude());
        }
        else 
        {
            let mut maxMagn = 0;
            for t1 in 0..trees.len() {
                for t2 in 0..trees.len() {
                    if t1 == t2 { continue; }
                        let mut tree = BinaryTree::<u8>::add(trees[t1].clone(), trees[t2].clone());
                        tree.reduce();
                        maxMagn = max(maxMagn, tree.magnitude());
                }
            }
            println!("{}", maxMagn);
        
        }
    }
}
