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

fn do_calc(mut write_file : &mut File, reader : &mut BufReader<File>, sequence : &Vec<&str>) -> i32 {
    let mut last = 0;
    let mut line = String::new();
    for i in 0..5 {
        //write!(&mut write_file, "1\n");
        write!(&mut write_file, "{}", &sequence[i]);
        write!(&mut write_file, "{}\n", last);

        reader.read_line(&mut line).expect("Unable to read line");
        last = line[..line.len()-1].parse::<i32>().unwrap();
        line.clear();
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


        let (parent_r,child_w) = pipe().unwrap();
        let (child_r, parent_w) = pipe().unwrap();
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                
                close(child_r).unwrap();
                close(child_w).unwrap();

                let mut write_file =  File::from_raw_fd(parent_w);
                let mut reader = BufReader::new( File::from_raw_fd(parent_r));
                 
                let mut sequence = vec!["0\n","1\n","2\n","3\n","4\n"];
                let mut permutations = Vec::new();
                heap_recursive(&mut sequence, |permutation| {
                    permutations.push(permutation.to_vec())
                 });
                let mut largest = 0;
                for s in permutations {
                    let value = do_calc(&mut write_file, &mut reader, &s);
                    println!("{}", value);
                    largest = std::cmp::max(largest,value);
                }
                println!("star1: {}",largest);
                close(parent_r).unwrap();
                close(parent_w).unwrap();
          
            }
            Ok(ForkResult::Child) => {
                close(parent_r).unwrap();
                close(parent_w).unwrap();
                close(nix::libc::STDIN_FILENO);
                close(nix::libc::STDERR_FILENO);

                dup2(child_r, nix::libc::STDIN_FILENO);
                dup2(child_w, nix::libc::STDOUT_FILENO);
                
                loop {
                    let mut pc = IntComp::new(&mut program, &ops);
                    pc.run();
                    io::stdout().flush().unwrap(); 
                }
                close(child_r).unwrap();
                close(child_w).unwrap();
            },
            Err(_) => println!("Fork failed"),
        }
    }
}
