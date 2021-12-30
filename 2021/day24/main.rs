use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use strum_macros::Display;
use std::collections::HashSet;

fn get_val(z : isize, offset : isize) -> isize {
    (z % 26) - offset
}

#[derive(PartialEq, Clone, Copy, Display, Debug)]
#[strum(serialize_all = "snake_case")]
enum Operation {
    NEQ,
    EQL,
    ADD,
    MUL,
    DIV,
    MOD,
    INP,
    SET,
    INVALID,
}

#[derive(PartialEq, Clone, Copy, Display, Debug)]
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

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

//debugging
//fn read_in_one_char() {
//    let input: Option<i32> = std::io::stdin()
//        .bytes()
//        .next()
//        .and_then(|result| result.ok())
//        .map(|byte| byte as i32);
//}

fn alu(instructions: &Vec<Instruction>, monad : &mut [isize;14]) -> isize
{

    let mut registers : [isize; 4] = [0;4];
    let mut i = 0;

    for ins in instructions {
        if !ins.apply(&mut registers) {

            let val = ins.set(&mut registers, monad[i], 13-i);
            if val <= 0 || val >= 10 { return -(13-i as isize)-1; }
            monad[i] = val;
            i += 1;
        }
    }

    registers[Register::Z as usize]
}

fn get_op(s : &str) -> Operation {
    match s {
        "eql" => Operation::EQL,
        "neq" => Operation::NEQ,
        "inp" => Operation::INP,
        "add" => Operation::ADD,
        "mul" => Operation::MUL,
        "div" => Operation::DIV,
        "mod" => Operation::MOD,
        _ => {panic!("unknown opcode");},
    }
}

fn get_symbol(op : Operation) -> String {
    match op {
         Operation::EQL => "==".to_string(),
         Operation::NEQ => "!=".to_string(),
         Operation::INP => "".to_string(),
         Operation::ADD => "+=".to_string(),
         Operation::MUL => "*=".to_string(),
         Operation::DIV => "/=".to_string(),
         Operation::MOD => "%=".to_string(),
         Operation::SET => "=".to_string(),
        _ => {panic!("panic");},
    }
}

#[derive(Debug,Clone)]
struct Instruction
{
    op : Operation,
    dst: Register,
    src : Register,
    val: isize,
}

fn bool_to_int(b : bool) -> isize {
    if b { return 1; }
    0
}
const Dummy : Instruction = Instruction{op : Operation::INVALID, dst : Register::INVALID, src : Register::INVALID, val : -1};
impl Instruction
{
    fn new(op : Operation, dst : Register, src : Register, val : isize) -> Self {
        Instruction{op : op, dst : dst, src : src, val : val}
    }
    fn parse(line : &str) -> Self {
        let parts : Vec<&str> = line.split_whitespace().collect();
        let dst =  get_reg(parts[1]);
        let op =  get_op(parts[0]);
        let mut val = -1;
        let mut src : Register = Register::INVALID;

        if parts.len() > 2 {
            src = get_reg(parts[2]);
            if src == Register::INVALID {
                val =  parts[2].parse().unwrap();
            }
        }
        Instruction{op : op, dst : dst, src : src, val : val}
    }

    fn set(&self, regs : &mut RegCache, val : isize, i : usize) -> isize {
        let v = match i {
            9 =>  get_val(regs[Register::Z as usize], 16),
            6 =>  get_val(regs[Register::Z as usize], 4),
            4 =>  get_val(regs[Register::Z as usize], 7),
            3 =>  get_val(regs[Register::Z as usize], 8),
            2 =>  get_val(regs[Register::Z as usize], 4),
            1 =>  get_val(regs[Register::Z as usize], 15),
            0 =>  get_val(regs[Register::Z as usize], 8),
            _ => val,
        };

        regs[self.dst as usize] = v;
        v
    }

    fn apply(&self, regs : &mut RegCache) -> bool {
        let mut op2 = self.val;
        if self.src != Register::INVALID{
            op2 = regs[self.src as usize];
        }

        let val = match self.op {
            Operation::INP => {return false;},
            Operation::ADD => regs[self.dst as usize] + op2,
            Operation::MOD => regs[self.dst as usize] % op2,
            Operation::DIV => regs[self.dst as usize] / op2,
            Operation::MUL => regs[self.dst as usize] * op2,
            Operation::EQL => bool_to_int(regs[self.dst as usize] == op2),
            Operation::NEQ=> bool_to_int(regs[self.dst as usize] != op2),
            _ => {panic!("invalid op");},
        };

        regs[self.dst as usize] = val;
        return true;
    }

    fn same_dst(&self, other : &Self) -> bool {
        self.dst == other.dst
    }

    fn nop(&self) -> bool {
        match self.op {
         Operation::DIV | Operation::MUL =>  if self.val == 1 { return true; },
         Operation::ADD =>  if self.val == 0 { return true; },
         _ => {}
        }
        false
    }

    fn set_zero(&self) -> bool {
         match self.op {
         Operation::MUL =>  if self.val == 0 { return true; },
         _ => {}
        }
        false
    }

    fn merge(&self, other : &Self) -> Self {
        if self.same_dst(other) {
            if self.set_zero() {
                if other.op == Operation::ADD {
                    return Instruction::new(Operation::SET, self.dst, other.src, other.val);
                }
            } else if self.op == Operation::EQL {
                if other.op == Operation::EQL && other.val == 0 {
                    return Instruction::new(Operation::NEQ, self.dst, self.src, self.val);
                }
            }
        }

        Dummy.clone()
    }

    fn valid(&self) -> bool {
        self.op != Operation::INVALID
    }

    fn is_cmp(&self) -> bool {
         self.op == Operation::EQL || self.op == Operation::NEQ 
    }

    fn print(&self)
    {
        if self.op == Operation::INP
        {
            println!("inp {}", self.dst);
        } else {
            if self.src == Register::INVALID {
                println!("{} {} {}", self.op, self.dst, self.val);
            } else {
                println!("{} {} {}", self.op, self.dst, self.src);
            }
        }
    }

    fn print_hr(&self, assign : bool) -> String
    {
        if self.op == Operation::INP
        {
            return format!("read &{}", self.dst);
        } else {
            let symbol = get_symbol(self.op);
            if assign || self.op == Operation::EQL || 
                self.op == Operation::NEQ || 
                    self.op == Operation::SET
            {
                if self.src == Register::INVALID {
                    return format!("{} {} {}", self.dst, symbol, self.val);
                } else {
                   return format!("{} {} {}", self.dst, symbol, self.src);
                }
            }
            else 
            {
                if self.src == Register::INVALID {
                    return format!(" {} {}", &symbol[0..1], self.val);
                } else {
                    return format!(" {} {}", &symbol[0..1], self.src);
                }
            }
        }
    }


}

fn print_instructions_hr(instructions : &Vec<Instruction>) 
{
    let mut i = 0;
    while i < instructions.len() {
        let ins = &instructions[i];
        match ins.op {
            Operation::NEQ | Operation::EQL =>{
                print!("{} = (", ins.dst);
                print!("{})", ins.print_hr(true));
                println!(" ? 1 : 0");
            },

            Operation::SET => {
                  print!("{}",ins.print_hr(true));

                    let mut j = i+1;
                    while j < instructions.len() && 
                        instructions[j].same_dst(&instructions[j-1]) &&
                        !instructions[j].is_cmp() {
                        print!("{}", instructions[j].print_hr(false));
                        j += 1;
                    }
                    i = j-1;
                    println!();
            },
            _ => println!("{}", ins.print_hr(true)),
        }
        i += 1;
    }
}
//9 9 9 9 3
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

        //TODO: rename let mut instructions to nextInstr
        //uncomment this to get optimized assembler output
        //see output txt for manually edited assembly
        //let mut instructions = vec![];

        //loop { if nextInstr.len() == instructions.len() { break; }

        //    instructions = nextInstr;
        //    nextInstr = vec![];

        //    let mut i = 0;
        //    while (i as isize)< (instructions.len() as isize) - 1 {
        //        let instr = instructions[i].merge(&instructions[i+1]);
        //        if instr.valid() {
        //            nextInstr.push(instr);
        //            i+=2;
        //        }
        //        else
        //        {
        //            nextInstr.push(instructions[i].clone());
        //            i+=1;
        //        }
        //    }
        //    if i < instructions.len() {
        //        nextInstr.push(instructions[i].clone());
        //    }
        //}
        //print_instructions_hr(&instructions);
        //process::exit(0);
        //
        //

        //check one specifc input
        //let mut monad1 : [isize;14] = [1,4,1,7,1,9,1,1,1,8,1,2,1,1];
        //println!("{}", alu(&instructions, &mut monad1));
        //let key1 : String = monad1.into_iter().map(|x| x.to_string()).collect();
        //println!("{}", key1);
        //process::exit(0);

        let star : u8 = 2;
        let mut checked = HashSet::new();
        let mut dir = -1;
        let mut start = 9;
        if star == 2 {
            dir = 1;
            start = 1;
        }
        let mut monad : [isize;14] = [start;14];
        let mut valid = 1;
        while valid != 0{
            valid = alu(&instructions, &mut monad);
            if valid < 0
            {   
                for i2 in ((-valid) as usize)..14 {
                    let i = 13 - i2;
                    if [13,13-1,13-2,13-3,13-4,13-6,13-9].contains(&i){ continue; }

                    if monad[i] == 10-start {
                        monad[i] = start;
                        continue;
                    }
                    monad[i] += dir;
                    let key : String = monad.into_iter().map(|x| x.to_string()).collect();
                    if checked.contains(&key) {
                        monad[i] += dir * -1;
                        continue;
                    }
                    checked.insert(key);

                    break;
                }
            }
        }
        let key : String = monad.into_iter().map(|x| x.to_string()).collect();
        println!("{}", key);
    }
}
