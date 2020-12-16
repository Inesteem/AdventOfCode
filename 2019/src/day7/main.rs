use lib::io::*;
pub mod intcomp;
use intcomp::*;
use std::process;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader,BufWriter,Read,Write};
use std::fs::{File};
use nix::unistd::*;
use std::os::unix::io::{RawFd,AsRawFd,FromRawFd};
use permutohedron::heap_recursive;


struct UglyPipe{
    parent_r : i32,
    parent_w : i32,
    child_r : i32,
    child_w : i32,
}
impl UglyPipe {

    fn new() -> Self {
        let (parent_r,child_w) = pipe().unwrap();
        let (child_r, parent_w) = pipe().unwrap();
        Self {
            parent_r : parent_r,
            parent_w : parent_w,
            child_r  : child_r ,
            child_w  : child_w ,
        }
    }

    fn close_childs(&mut self){
       close(self.child_r).unwrap(); 
       close(self.child_w).unwrap(); 
 }
    fn close_parents(&mut self){
       close(self.parent_r).unwrap(); 
       close(self.parent_w).unwrap(); 
    } 

    fn dup_childs(&mut self) {
        close(nix::libc::STDIN_FILENO).unwrap();
        close(nix::libc::STDERR_FILENO).unwrap();
        dup2(self.child_r, nix::libc::STDIN_FILENO).unwrap();
        dup2(self.child_w, nix::libc::STDOUT_FILENO).unwrap();

    }
}

fn do_calc(write_files : &mut Vec<File>, readers : &mut Vec<BufReader<File>>, sequence : &Vec<i32>) -> i32 {
    let mut last = 0;
    let mut line = String::new();
    println!("s: {:?}", sequence); 
    for i in 0..5 {
        write!(&mut write_files[i], "{}\n", &sequence[i]).unwrap();
    }

    let mut fin = false;
    while !fin {
        for i in 0..5 {
            write!(&mut write_files[i], "{}\n", last).unwrap();
            readers[i].read_line(&mut line).expect("Unable to read line");
            println!("line{}: >{}< ({})",i,line.trim(),last);
            if line.len() == 0  || line.eq("fin\n") {
                fin = true;
                line.clear();
                continue;
            }
            last = line[..line.len()-1].parse::<i32>().unwrap();
            line.clear();
        }
    }
    last
}

fn main() {

    let ops : Vec< (usize, Box<dyn Fn(&mut Vec<i32>,i32,i32,i32,usize) -> usize>) > = vec![
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize] = o1 + o2; return 4+i})),
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize] = o1 * o2; return 4+i})),
        (1,Box::new(|v,o1,_,_,i|   { v[o1 as usize] = read_in_int(); return 2+i})),
        (1,Box::new(|_,o1,_,_,i|   { println!("{}", o1); return 2+i})),
        (2,Box::new(|_,o1,o2,_,i|  { if o1 != 0    {return o2 as usize;} return 3+i})),
        (2,Box::new(|_,o1,o2,_,i|  { if o1 == 0    {return o2 as usize;} return 3+i})),
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize ] = 0; if o1 < o2   {v[o3 as usize] = 1;} return 4+i})),
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize ] = 0; if o1 == o2  {v[o3 as usize] = 1;} return 4+i})),
    ];

    let mut program: Vec<i32> = read_line_from_stdin().unwrap().trim().split(",")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();
    unsafe {

        let mut pipes = vec![];
        for i in 0..5 {
            pipes.push(UglyPipe::new());
        }
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                let mut write_files = vec![];
                let mut readers= vec![];
                for p in &mut pipes { 
                    p.close_childs();  
                    write_files.push(File::from_raw_fd(p.parent_w));
                    readers.push(BufReader::new(File::from_raw_fd(p.parent_r)));
                } 
                //let mut sequence = vec![0,1,2,3,4];
                let mut sequence = vec![5,6,7,8,9];
                let mut permutations = Vec::new();
                heap_recursive(&mut sequence, |permutation| {
                    permutations.push(permutation.to_vec())
                 });
                let mut largest = 0;
                for s in permutations {
                    //let s = vec![9, 8, 7, 6, 5];
                    //let s = vec![7,5,6,8,9];
                    let value = do_calc(&mut write_files, &mut readers, &s);
                    println!("val: {}", value);
                    largest = std::cmp::max(largest,value);
                }
                println!("star2: {}",largest);
          
            }
            Ok(ForkResult::Child) => {
                for p in &mut pipes { p.close_parents(); } 
                for ip in 0..pipes.len(){

                    match fork() {
                    Ok(ForkResult::Parent { child, .. }) => {
                    },
                    Ok(ForkResult::Child) => {
                       // for j in 0..5 {
                       //     if i == j {continue;}
                       //     pipes[j].close_childs();
                       // }
                        pipes[ip].dup_childs();                        
                        loop {
                            let mut pc = IntComp::new(&mut program, &ops);
                             pc.run();
                            println!("fin");
                            read_in_int();
                            io::stdout().flush().unwrap(); 
                        }
                        println!("err");
                    },
                    Err(_) => println!("Fork failed"),
                    }
                }
                    //close(child_r).unwrap();
                //close(child_w).unwrap();
            },
            Err(_) => println!("Fork failed"),
        }
    }
}
//too high : 263462987
