use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use strum_macros::Display;

#[derive(PartialEq, Clone, Copy)]
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
enum Operation {
    NEQ,
    EQL,
    ADD,
    MUL,
    DIV,
    MOD,
    INP,
}

#[derive(PartialEq, Clone, Copy)]
#[derive(Display)]
#[strum(serialize_all = "snake_case")]
enum Register {
    W=0,
    X=1,
    Y=2,
    Z=3,
    INVALID,
}

type RegCache = [isize;4];

fn get_reg(c : &str) -> Register {
    match c {
         "w" => Register::W,
         "x" => Register::X,
         "y" => Register::Y,
         "z" => Register::Z,
        _ => Register::INVALID,
    }
}

fn get_val(c : &str, regs:&RegCache) -> isize
{
    let reg = get_reg(c);
    if reg == Register::INVALID 
    {
        return c.parse().unwrap();
    }
    regs[reg as usize]
}

fn add(r1:Register, r2:Register) -> impl Fn(&mut RegCache) {
    move |regs : &mut RegCache| {
            let val = regs[r2 as usize];
            regs[r1 as usize] += val;
    }
}
fn addi(r1:Register, val : isize) -> impl Fn(&mut RegCache) +  'static{
    move |regs : &mut RegCache| {regs[r1 as usize] += val;}
}
//
//fn sub(x: isize) -> impl Fn(isize)-> isize {
//    return move |y| x - y;
//}
//
//fn modi(x: isize) -> impl Fn(isize)-> isize {
//    return move |y| x % y;
//}
//
//fn div(x: isize) -> impl Fn(isize)-> isize {
//    return move |y| x / y;
//}
//
//fn mul(x: isize) -> impl Fn(isize)-> isize {
//    return move |y| x * y;
//}
//
//fn eql(x: isize) -> impl Fn(isize)-> isize {
//    return move |y| if x == y {return 1;} else {return 0};
//}
//
//
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

////fn apply(registers : &[isize; 4], c : &str, op : &dyn Fn(isize) -> isize) -> isize {
////   let idx = get_idx(c);
////   let mut val : isize = 0;
////
////   if idx < 0 { val = c.parse().unwrap(); }
////   else { val = registers[idx as usize]; }
////
////    op(val)
////}
////fn inp() -> isize {
////    std::io::stdin().lock().lines().next().expect("").expect("").parse().unwrap()
////}
//
//fn inp(num: &[isize;14]) -> impl Fn(usize) -> isize + '_ {
//    return move |idx| num[idx];
//}
//
////fn alu(instructions: &Vec<Vec<&str>>, input: &dyn Fn(usize) -> isize) -> isize
////{
////    let mut registers : [isize; 4] = [0;4];
////    let mut i : isize = -1;
////    for line in instructions {
////        let mut idx = get_idx(line[1]) as usize;
////        let op1 = registers[idx];
////        
////        let val = match line[0] {
////            "inp" => {i += 1; input(i as usize)}, 
////            "add" => apply(&registers, line[2], &add(op1)),
////            "sub" => apply(&registers, line[2], &sub(op1)),
////            "mod" => apply(&registers, line[2], &modi(op1)),
////            "div" => apply(&registers, line[2], &div(op1)),
////            "mul" => apply(&registers, line[2], &mul(op1)),
////            "eql" => apply(&registers, line[2], &eql(op1)),
////            _=> { panic!("not implemented {}", line[0]) }
////        };
////
////        registers[idx] = val;
////    }
////
////    registers[get_idx(&"z") as usize]
////}
//
fn get_op(s : &str) -> Operation {
    match s {
        "eql" => Operation::EQL,
        "neq" => Operation::NEQ,
        "inp" => Operation::INP,
        "add" => Operation::ADD,
        "mul" => Operation::MUL,
        "div" => Operation::DIV,
        _ => {panic!("panic");},
    }
}

static inputIdx: isize = -1;
struct Instruction
{
    op : Operation,
    dst: Register,
    src : isize,
    func : dyn Fn(&mut RegCache),
}

impl Instruction
{
    fn new(line : &str, regs : &mut RegCache) -> Self {
        let parts : Vec<&str> = line.split_whitespace().collect();
        let dst =  get_reg(parts[1]);
        let op =  get_op(parts[0]);

        //if op == Operation::INP {
        //    inputIdx += 1;
        //    return Instruction{op : op, dst : dst, src : -1, func : &addi(dst, 1)};//&input(inputIdx)};
        //}
        let src : isize =  parts[2].parse().unwrap();

        //let mut func = match op {
        //    Operation::ADD => &add(dst1)),
        //    //Operation::SUB => &sub(dst1)),
        //    //Operation::MOD => &modi(dst1)),
        //    //Operation::DIV => &div(dst1)),
        //    //Operation::MUL => &mul(dst1)),
        //    //Operation::EQL => &eql(dst1)),
        //    //Operation::NEQ => &eql(dst1)),
        //    _=> { panic!("not implemented {}", line[0]); }
        //};

        //Instruction{op : op, dst : dst, src : src, func}
            Instruction{op : op, dst : dst, src : -1, func : addi(dst, 1)}

    }


    fn print(&self) 
    {
        if self.op == Operation::INP 
        {
            println!("inp w"); 
        } else {
            println!("{} {} {}", self.op, self.src, self.dst);
        }
    }
}
//
//fn same_dst(ins1 : &Vec<&str>, ins2 : &Vec<&str>) -> bool {
//    ins1[1] == ins2[1]
//}
//
//fn same_cross_var(ins1 : &Vec<&str>, ins2 : &Vec<&str>) -> bool {
//    if ins2.len() != 3 { return false; }
//    ins1[1] == ins2[2]
//}
//
////fn skip(ins : &Vec<&str>) -> bool {
////    if ins.len() == 3 {
////        if ins[0] == "div" && ins[2] == "1" {
////            return true;
////        }
////    }
////    false
////}
////fn simplify(ins1 : &Vec<&str>, ins2 : &Vec<&str>) -> bool{
////    if same_dst(ins1,ins2) {
////        match op(ins1) {
////            "set" => {
////
////                match( op(ins2) ) {
////
////
////            }
////                    
////                    == "add" {
////                    println!("set {} {}+{}", dst(ins1), src(ins1), src(ins2));
////                    return true;
////                }
////                if op(ins2) == "mul" {
////                    println!("set {} {}*{}", dst(ins1), src(ins1), src(ins2));
////                    return true;
////                }
////                if op(ins2) == "div" {
////                    println!("set {} {}/{}", dst(ins1), src(ins1), src(ins2));
////                    return true;
////                }
////                if op(ins2) == "mod" {
////                    println!("set {} {}%{}", dst(ins1), src(ins1), src(ins2));
////                    return true;
////                }
////                if op(ins2) == "neq" {
////                    println!("set {} 1 if {} != {} else 0", src(ins1), dst(ins1), dst(ins2));
////                    return true;
////                }
////            },
////            "mul" => {
////                if dst(ins1) == "0" && op(ins2) == "add" {
////                    println!("set {} {}", src(ins2), dst(ins2));
////                    return true;
////                }
////            },
////            "eql" => {
////                if op(ins2) == "eql" && dst(ins2) == "0" {
////                    println!("neq {} {}", src(ins1), dst(ins1));
////                    return true;
////                }
////            },
////            _ => {},
////        }
////    }
////    else if same_cross_var(ins1,ins2) { 
////        match op(ins1) {
////            "set" => {
////                if op(ins2) == "mul" {
////                    println!("set {} {}*{}", src(ins2), src(ins2), dst(ins1));
////                    return true;
////                }
////                if op(ins2) == "div" {
////                    println!("set {} {}/{}", src(ins2), src(ins2), dst(ins1));
////                    return true;
////                }
////                if op(ins2) == "mod" {
////                    println!("set {} {}%{}", src(ins2), src(ins2), dst(ins1));
////                    return true;
////                }
////                if op(ins2) == "add" {
////                    println!("set {} {}+{}", src(ins2), src(ins2), dst(ins1));
////                    return true;
////                }
////            },
////            _ => {},
////        }
////    }
////
////    println!("{}", ins1.join(" "));
////    false
////}
fn main() {
//
//    let files = vec!["data"];
//    for file in files {
//        let input: String;
//        match read_inputs(file.to_string()) {
//            Ok(inputs) =>
//                input = inputs,
//            Err(_) => continue,
//        }
//
//
        
//        let lines : Vec<Instruction>= input.lines().
//            map(|line| Instruction::new(line, &mut regs)).
//                collect();
//
//
//
//        //let mut i = 0;
//        //while i < lines.len()-1 {
//        //    if skip(&lines[i]) {
//        //        i+=1;
//        //        continue;
//        //    }
//        //    if simplify(&lines[i], &lines[i+1]) {i+=2;}
//        //    else { i+=1; }
//        //}
//        //process::exit(0);
//
//        //let mut monad : [isize;14] = [3;14];
//        //let mut valid = 1;
//        //while valid != 0{
//        //    valid = alu(&lines, &inp(&monad));
//
//        //    for i1 in 0..14 {
//        //        let i = 13 - i1;
//        //        if monad[i] == 1 {
//        //            monad[i] = 9;
//        //            continue;
//        //        }
//        //        monad[i] -= 1;
//        //        break;
//        //    }
//        //    println!("{} {:?}", valid, monad);
//        //    read_in_one_char();
//        //}
//
//    }
}
