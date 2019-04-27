use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_idx(s: &str, max: i32) -> i32{
    let v: Vec<char> = s.chars().collect();
    if v.len() != 1{
        return -1;
    }
    let idx = (v[0].to_digit(36).unwrap() as i32) - 10;
    if idx < 0 || idx >  max {
        return -1;
    }
    idx 
}

#[derive(Debug)]
enum Assembunny{
    CPY_R_R(usize, usize),
    CPY_V_R(i32, usize),
    JNZ_V(i32,i32),
    JNZ_R(usize,i32),
    INC(usize),
    DEC(usize),
}

impl Assembunny {
    fn exec(&self , regs: &mut [i32], ip: &mut i32) {
        match self {
            Assembunny::CPY_R_R(r_src,r_dst) => regs[*r_dst] = regs[*r_src], 
            Assembunny::CPY_V_R(val,r_dst)   => regs[*r_dst] = *val, 
            Assembunny::INC(r)               => regs[*r] += 1, 
            Assembunny::DEC(r)               => regs[*r] -= 1, 
            Assembunny::JNZ_R(r,step)        => if regs[*r] != 0 {*ip += *step; return;} , 
            Assembunny::JNZ_V(val,step)      => if *val != 0 {*ip += *step; return;} , 
        }
        *ip += 1;
    }

    fn print(&self){
        match self {
            Assembunny::CPY_R_R(r_src,r_dst) => println!("cpy_r_r : {} -> {}", r_src, r_dst), 
            Assembunny::CPY_V_R(val,r_dst)   => println!("cpy_v_r : {} -> {}", val, r_dst), 
            Assembunny::INC(r)               => println!("inc {}",r), 
            Assembunny::DEC(r)               => println!("dec {}",r), 
            Assembunny::JNZ_R(r,step)        => println!("jnz_r {} {}",r,step), 
            Assembunny::JNZ_V(val,step)      => println!("jnz_v {} {}",val,step), 
        }

    }
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split_str = contents.split("\n");

    let mut regs = [0; 4];
    regs[2] = 1;
    let mut ip: i32 = 0;


    let mut commands: Vec<Assembunny> = Vec::new();

    for s in split_str {
        let com = s.split(" ").collect::<Vec<&str>>();
        if com.len() < 2{
            continue;
        }
        let r = get_idx(com[1], regs.len() as i32);
        match com[0] {
            "inc" =>    commands.push(Assembunny::INC(r as usize )) ,
            "dec" =>    commands.push(Assembunny::DEC(r as usize )) ,
            "jnz" =>   
                if r != -1 {
                    commands.push(Assembunny::JNZ_R(r as usize, com[2].parse::<i32>().unwrap()));
                } else {
                    commands.push(Assembunny::JNZ_V(com[1].parse::<i32>().unwrap(), com[2].parse::<i32>().unwrap()));
                },


            "cpy" =>   
                if r != -1 {
                    commands.push(Assembunny::CPY_R_R(r as usize, get_idx(com[2], regs.len() as i32) as usize));
                } else {
                    commands.push(Assembunny::CPY_V_R(com[1].parse::<i32>().unwrap(), get_idx(com[2], regs.len() as i32) as usize));
                },

            _ => assert!(false),
        }
    }

    while ip >= 0 && ip < commands.len() as i32{
        let c = &commands[ip as usize ];
        //c.print();
        c.exec(&mut regs, &mut ip);
        //println!("{:?}", regs);
    }

    println!("{:?}", regs);

}
