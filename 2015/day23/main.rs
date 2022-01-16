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
}

#[derive(PartialEq, Clone, Copy, Display, Debug)]
#[strum(serialize_all = "snake_case")]
enum Register {
    A=0,
    B=1,
    INVALID,
}

type RegCache = [isize;2];

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

fn alu(instructions: &Vec<Instruction>) -> isize
{

    let mut registers : RegCache = [0;2];
    let mut i = 0;

    while i < instructions.len() {
        i = ins.apply(&mut registers);
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

const Dummy : Instruction = Instruction{op : Operation::INVALID, dst : Register::INVALID, src : Register::INVALID, val : -1};
impl Instruction
{
    fn new(op : Operation, dst : Register, val : isize) -> Self {
        Instruction{op : op, reg : reg,  val : val}
    }
    fn parse(line : &str) -> Self {
        let parts : Vec<&str> = line.split_whitespace().collect();

        //REG?
        let reg = get_reg(parts[1][0..1]);
        if src == Register::INVALID {
            val =  parts[1].parse().unwrap();
        }

        let mut val = -1;
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
            Operation::JMP => {return val},
            Operation::JIE => {if regs[self.reg as usize] % 2 == 0 { return val;} return 1;} 
            Operation::JIO => {if regs[self.reg as usize] == 1 { return val;} return 1;} 
            _ => {panic!("invalid op");},
        };

        regs[self.dst as usize] = val;
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
            filter(|i| !i.nop()).
            collect();
        
        let regB = alu(&instructions);
        println!("{}", regB);
    }
}
