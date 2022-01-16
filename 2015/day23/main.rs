use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use strum_macros::Display;
use std::collections::HashSet;

#[derive(PartialEq, Clone, Copy, Display, Debug)]
#[strum(serialize_all = "snake_case")]
enum Operation {
    HLF,
    TPL,
    INC,
    JMP,
    JIE,
    JIO,
    INVALID,
}

#[derive(PartialEq, Clone, Copy, Display, Debug)]
#[strum(serialize_all = "snake_case")]
enum Register {
    A=0,
    B=1,
    INVALID,
}

type RegCache = [usize;2];

fn get_reg(c : &str) -> Register {
    match c {
         "a" => Register::A,
         "b" => Register::B,
        _ => Register::INVALID,
    }
}

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}
const star2 : bool = true;
fn alu(instructions: &Vec<Instruction>) -> usize
{

    let mut registers : RegCache = [0;2];
    if star2 { registers[0] = 1;}
    let mut i : isize = 0;

    while i < instructions.len() as isize  && i >= 0 {
        i += instructions[i as usize].apply(&mut registers);
        println!("{} : {:?}", i, registers);
    }

    registers[Register::B as usize]
}

fn get_op(s : &str) -> Operation {
    match s {
        "hlf" => Operation::HLF,
        "tpl" => Operation::TPL,
        "inc" => Operation::INC,
        "jmp" => Operation::JMP,
        "jie" => Operation::JIE,
        "jio" => Operation::JIO,
        _ => {panic!("unknown opcode");},
    }
}

#[derive(Debug,Clone)]
struct Instruction
{
    op  : Operation,
    reg : Register,
    val : isize,
}

const Dummy : Instruction = Instruction{op : Operation::INVALID, reg : Register::INVALID, val : -1};
impl Instruction
{
    fn new(op : Operation, reg : Register, val : isize) -> Self {
        Instruction{op : op, reg : reg,  val : val}
    }
    fn parse(line : &str) -> Self {
        let parts : Vec<&str> = line.split_whitespace().collect();
        let mut val = -1;
        let op =  get_op(parts[0]);
        //REG?
        let reg = get_reg(&parts[1][0..1]);
        if reg == Register::INVALID {
            val =  parts[1].parse().unwrap();
        }

        if parts.len() > 2 {
            val = parts[2].parse().unwrap();
        }
        Instruction{op : op, reg : reg, val : val}
    }

    fn apply(&self, regs : &mut RegCache) -> isize {

        let val = match self.op {
            Operation::HLF => regs[self.reg as usize] / 2,
            Operation::TPL => regs[self.reg as usize] * 3,
            Operation::INC => regs[self.reg as usize] + 1,
            Operation::JMP => {return self.val},
            Operation::JIE => {if regs[self.reg as usize] % 2 == 0 { return self.val;} return 1;} 
            Operation::JIO => {if regs[self.reg as usize] == 1 { return self.val;} return 1;} 
            _ => {panic!("invalid op");},
        };

        regs[self.reg as usize] = val;
        return 1;
    }

    fn valid(&self) -> bool {
        self.op != Operation::INVALID
    }
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

        let mut instructions: Vec<Instruction>= input.lines().
            map(|line| Instruction::parse(line)).
            collect();
       
        for i in &instructions {
            println!("{:?}", i);
        }
        let regB = alu(&instructions);
        println!("{}", regB);
    }
}
