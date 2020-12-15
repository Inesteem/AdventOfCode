
pub struct Opcode{
    pub instr : u8,
    pub mode_1: bool,
    pub mode_2: bool,
    pub mode_3: bool 
}

impl Opcode {

    pub fn new() -> Opcode {
        Opcode{
            instr: 0,
            mode_1 : false,
            mode_2 : false,
            mode_3 : false
        }
    }

    pub fn parse(&mut self, opcode : i32){
        let digits: Vec<_> = opcode.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        let len = digits.len();
        self.instr = digits[len - 1] as u8;
        if len >= 2 {
            self.instr += (digits[len - 2] * 10) as u8;
        } 
        
        if len >= 3 {
            self.mode_1 = digits[len-3] == 1;
        } else { 
            self.mode_1 = false;
        }
        
        if len >= 4 {
            self.mode_2 = digits[len-4] == 1;
        } else { 
            self.mode_2 = false;
        }
        
        if len >= 5 {
            self.mode_3 = digits[len-5] == 1;
        } else { 
            self.mode_3 = false;
        }
    }
}


pub struct IntComp<'a> {
    program : Vec<i32>,
    opc : Opcode,
    ops : &'a Vec< (usize, Box<dyn Fn(&mut Vec<i32>,i32,i32,i32,usize) -> usize>) >,
} 
impl<'a> IntComp<'a> {
    
    pub fn new(program : & Vec<i32>, ops: &'a Vec< (usize, Box<dyn Fn(&mut Vec<i32>,i32,i32,i32,usize) -> usize>) >) -> Self {
        Self {
            program : program.clone(),
            opc : Opcode::new(),
            ops  : ops, 
        }
    }

    pub fn run(&mut self) {
        let mut i : usize = 0;
        while i < self.program.len() {
            if  self.program[i] == 99  {
                break;
            }

            i = self.do_op(i);

        }
    }



    pub fn do_op(&mut self, i : usize) -> usize {
      let opc = &mut self.opc;
      let ops = & self.ops;
      let program = &mut self.program;
      let mut o1 = program[i+1];
      let mut o2 : i32 = 0;
      let mut o3 : i32 = 0;

      opc.parse(program[i]);
      let num_params = ops[(opc.instr - 1) as usize].0;

 //     if num_params == 1 {
 //         println!("{} {}", program[i] , program[i+1]);
 //     }
 //     if num_params == 2 {
 //         println!("{} {} {}", program[i] , program[i+1], program[i+2]);
 //     }
 //     if num_params == 3 {
 //         println!("{} {} {} {}", program[i] , program[i+1], program[i+2], program[i+3]);
 //     }
      opc.mode_1 = (opc.mode_1 || opc.instr == 3);
      if !opc.mode_1 {
          o1 = program[o1 as usize];
      }  
       if num_params > 1 {
          o2 = program[i+2];
          if !opc.mode_2 {
              o2 = program[o2 as usize];
          }  
      } if num_params > 2 {
          o3 = program[i+3];
      }

      return ops[(opc.instr - 1) as usize].1(program, o1, o2, o3, i);
    }

}
