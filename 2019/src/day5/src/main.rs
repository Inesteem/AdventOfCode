use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::io;





struct Opcode{
    instr : u8,
    mode_1: bool,
    mode_2: bool,
    mode_3: bool 
}

impl Opcode {

    fn new() -> Opcode {
        Opcode{
            instr: 0,
            mode_1 : false,
            mode_2 : false,
            mode_3 : false
        }
    }

    fn parse(&mut self, opcode : i32){
        let digits: Vec<_> = opcode.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        let len = digits.len();
        self.instr = digits[len - 1] as u8;
        if len >= 2 {
            self.instr += (digits[len - 2] * 10) as u8;
        } 
        
        if len >= 3 {
            self.mode_1 = (digits[len-3] == 1);
        } else { 
            self.mode_1 = false;
        }
        
        if len >= 4 {
            self.mode_2 = (digits[len-4] == 1);
        } else { 
            self.mode_2 = false;
        }
         
        
        if len >= 5 {
            self.mode_3 = (digits[len-5] == 1);
        } else { 
            self.mode_3 = false;
        }
 //       println!("{:?} {} {} {} {}", digits, self.mode_3, self.mode_2, self.mode_1, self.instr);
    }


//
//    Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//    Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//    Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//    Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//

    fn do_op(&mut self, program: &mut Vec<i32>, i : usize, ops: & Vec<(usize, Box<dyn Fn(&mut Vec<i32>,i32,i32,i32, usize) -> usize > )> ) -> usize {
      self.parse(program[i]);
      let mut o1 = program[i+1];
      let mut o2 : i32 = 0;
      let mut o3 : i32 = 0;

      let num_params = ops[(self.instr - 1) as usize].0;

 //     if num_params == 1 {
 //         println!("{} {}", program[i] , program[i+1]);
 //     }
 //     if num_params == 2 {
 //         println!("{} {} {}", program[i] , program[i+1], program[i+2]);
 //     }
 //     if num_params == 3 {
 //         println!("{} {} {} {}", program[i] , program[i+1], program[i+2], program[i+3]);
 //     }
      self.mode_1 = (self.mode_1 || self.instr == 3);
      if !self.mode_1 {
          o1 = program[o1 as usize];
      }  
       if num_params > 1 {
          o2 = program[i+2];
          if !self.mode_2 {
              o2 = program[o2 as usize];
          }  
      } if num_params > 2 {
          o3 = program[i+3];
//          if !self.mode_3 && self.instr < 6{
//              o3 = program[o3 as usize];
//          }  
      }



      //if self.instr > 6{
      //    if !self.mode_3 {
      //        o3 = program[o3 as usize];
      //    }  
      //}
  //    println!("{} {} {}", o1, o2, o3);
      return ops[(self.instr - 1) as usize].1(program, o1, o2, o3, i);
    }
}

fn read_in_int() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    return -1
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
    let mut opc = Opcode::new();

    let ops : Vec< (usize, Box<dyn Fn(&mut Vec<i32>,i32,i32,i32,usize) -> usize>) > = vec![
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize] = o1 + o2; return 4+i})),
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize] = o1 * o2; return 4+i})),
        (1,Box::new(|v,o1,_,_,i|   { print!("input  : "); io::stdout().flush().unwrap(); v[o1 as usize] = read_in_int(); return 2+i})),
        (1,Box::new(|v,o1,_,_,i|   { println!("output : {}", o1); return 2+i})),
        (2,Box::new(|v,o1,o2,_,i|  { if o1 != 0    {return o2 as usize;} return 3+i})),
        (2,Box::new(|v,o1,o2,_,i|  { if o1 == 0    {return o2 as usize;} return 3+i})),
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize ] = 0; if o1 < o2   {v[o3 as usize] = 1;} return 4+i})),
        (3,Box::new(|v,o1,o2,o3,i| { v[o3 as usize ] = 0; if o1 == o2  {v[o3 as usize] = 1;} return 4+i})),
    ];


    let mut input_vec : Vec<i32>;
    match read_inputs("../../data/day5.txt".to_string()) {
        Ok(inputs) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
            input_vec = inputs.split(",")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect(),
        Err(_) => process::exit(0),
            
    }
    println!("");
    let mut program = input_vec;
//    program = vec![3,9,8,9,10,9,4,9,99,-1,8];
//    program = vec![3,9,7,9,10,9,4,9,99,-1,8];
//    program = vec![3,3,1108,-1,8,3,4,3,99];
//    program = vec![3,3,1107,-1,8,3,4,3,99];
//    program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
//    program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
//    program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104, 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
//

    let mut i : usize = 0;
    while i < program.len() {
//        println!("{:?}", program);        
        if  program[i] == 99  {
            break;
        }

        i = opc.do_op(&mut program, i, & ops);

    }

}
