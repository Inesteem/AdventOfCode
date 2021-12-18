use std::fs::File;
use std::io::BufReader;
use std::process;
use std::io::Read;
use std::i64;
use std::usize;
use std::cmp::min;
use std::cmp::max;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn print_stream( stream : &[u8]) {
    let sstream : String = stream.iter().map(|x| x.to_string()).collect();
    println!("{}",sstream);
}

fn to_num(n : &[u8]) -> u16
{
    let mut num : u16 = 0;
    for c in n {
        num <<= 1;
        num |= *c as u16;
    }
    num
}

trait Parse {
    fn parse (s : &[u8]) -> Self;
}

#[derive(Debug)]
struct Packet {
   version : u16,
   typeId  : u16,
   packets : Vec<Packet>,
   literal : usize,
   size : usize,
}

trait Eval {
    fn cnt_vnums(&self) -> usize;
    fn eval(&self) -> usize;
}

impl Eval for Packet {
    fn cnt_vnums(&self) -> usize {
        let mut v = self.version as usize;
        for i in 0..self.packets.len() {
            v += self.packets[i].cnt_vnums();
        }
        v
    }

    fn eval(&self) -> usize {
        match self.typeId {
            0 => {
                let mut e = 0;
                for p in &self.packets {
                    e += p.eval();
                }
                e
            }
            1 => {
                let mut e = 1;
                for p in &self.packets {
                    e *= p.eval();
                }
                e
            }
            2 => {
                let mut e = usize::MAX;
                for p in &self.packets {
                    e = min(p.eval(), e);
                }
                e
            }
            3 => {
                let mut e = usize::MIN;
                for p in &self.packets {
                    e = max(p.eval(), e);
                }
                e
            }
            5 => {
                if self.packets[0].eval() > self.packets[1].eval() { return 1;}
                0
            }
            6 => {
                if self.packets[0].eval() < self.packets[1].eval() { return 1;}
                0
            }
            7 => {
                if self.packets[0].eval() == self.packets[1].eval() { return 1;}
                0
            }
            _ => self.literal
        }
    }

}
impl Parse for Packet {
    fn parse (bin : &[u8]) -> Packet {
        let version = to_num(&bin[0..3]);
        let typeId = to_num(&bin[3..6]);
        if typeId == 4 { // literal
            let mut pos = 6;
            let mut literal : String= String::new();
            loop {
                let n = to_num(&bin[pos+1..pos+5]);
                let c = format!("{:x}", n);
                literal.push_str(&c);

                if bin[pos] == 0 {
                    break;
                }
                pos+=5;
            }
            return Packet{version : version, typeId : typeId, packets: vec![], literal : usize::from_str_radix(&literal, 16).unwrap(), size : pos+5};
        }
        //operator
        let mut packets = vec![];
        let I = bin[6];

        let mut start = 7;
        let mut pos = 0;
        if I == 0 {
            let length = to_num(&bin[start..start+15]) as usize;
            start += 15;
            pos = start;


            while pos < start + length && (start + length) - pos > 6 {
                let p = Packet::parse(&bin[pos..start+length]);
                pos += p.size;

                packets.push(p);
            }
        } else {
            let length = to_num(&bin[start..start+11]) as usize;
            start += 11;
            pos = start;

            for p in 0..length {
                let p = Packet::parse(&bin[pos..bin.len()]);
                pos += p.size;

                packets.push(p);
            }
        }
        Packet{version : version, typeId : typeId, packets: packets, literal : 0, size : pos}
    }
}

fn get_binary(line : &str) -> Vec<u8> {

    let mut bin = vec![0; line.len()*4];

    for i in 0..line.len() {
       let mut z = i64::from_str_radix(&line[i..i+1], 16).unwrap();
       for b in 0..4 {
            if z & 0x1 == 0 {
                bin[i * 4 + 3 - b] = 0
            } else {
                bin[i * 4 + 3 - b] = 1;
            }
            z >>= 1;
       }

    }
    bin

}


fn main() {
//    let files = vec!["test3"];
    let files = vec!["test6", "data"];

    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<&str> = input.lines().collect();
        for line in lines {
            println!("{}", line);
            let bin = get_binary(&line);
            let p = Packet::parse(&bin);
            //println!("{:?}", p);
            println!("star1: {}", p.cnt_vnums());
            println!("star2: {}", p.eval());
        }

    }
}
