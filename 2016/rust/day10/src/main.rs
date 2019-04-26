use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Command {
    bot_from: usize,
    bot_low:  Drain,
    bot_high: Drain,
}

#[derive(Debug)]
enum Drain{
    Bot(usize),
    Output(usize),
}

impl Command {
    fn new(com: &Vec<&str>)-> Command {
        Command {
            bot_from: com[1].parse::<usize>().unwrap(),
            bot_low:  match com[5]{
                "bot" => Drain::Bot(com[6].parse::<usize>().unwrap()),
                _ => Drain::Output(com[6].parse::<usize>().unwrap()),
            },
            bot_high:  match com[10]  {
                "bot" => Drain::Bot(com[11].parse::<usize>().unwrap()),
                _ => Drain::Output(com[11].parse::<usize>().unwrap()),
            }

        }
    }

    fn execute(&self, bots: &mut HashMap<usize, Vec<usize>>, commands: & HashMap<usize, Command>, outputs: & HashMap<usize, Vec<usize>>){

        let val_low: usize;
        let val_high: usize;
        {
            //assert!(bots.get_mut(&self.bot_from) != None);
            //let bot_from = bots.entry(self.bot_from).or_insert(vec![0,1]);
            let bot_from = bots.get_mut(&self.bot_from).unwrap();
            if bot_from.len() != 2 {
                return;
            }
            assert!(bot_from.iter().min() != None);
            assert!(bot_from.iter().max() != None);
            val_low = *(bot_from.iter().min().unwrap());
            val_high = *(bot_from.iter().max().unwrap());
            //code for star 1
            //if val_low == 17 && val_high == 61 {
            //    println!("bot responsibel for comparison: {}", self.bot_from);
            //    panic!("finished!");
            //}

            bot_from.clear();

        }
        assert!(bots.get(&self.bot_from).unwrap().len() == 0);

        if let Drain::Bot(bot) =  self.bot_low {
            let bot_low = bots.entry(bot).or_insert(Vec::new());
            assert!(bot_low.len() < 2);
            bot_low.push(val_low);
            println!("bot {} gives low ({}) to bot {}", self.bot_from, val_low, bot);
        } else {
            println!("bot {} gives low ({}) to output", self.bot_from, val_low);
        }
        if let Drain::Bot(bot) =  self.bot_high {
            let bot_high = bots.entry(bot).or_insert(Vec::new());
            assert!(bot_high.len() < 2);
            bot_high.push(val_high);
            println!("bot {} gives high ({}) to bot {}", self.bot_from, val_high, bot);
        } else {
            println!("bot {} gives high ({}) to output", self.bot_from, val_high);
        }   
       
        if let Drain::Bot(bot) =  self.bot_low {
            commands.get(&bot).unwrap().execute(bots, &commands, &outputs);
        }   
        if let Drain::Bot(bot) =  self.bot_high {
            commands.get(&bot).unwrap().execute(bots, &commands, &outputs);
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

    let mut inputs: Vec<(usize,usize)> = Vec::new();
    let mut bots: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut commands: HashMap<usize, Command> = HashMap::new();
    let mut outputs: HashMap<usize, Vec<usize>> = HashMap::new();

    let split_str = contents.split("\n");
    for s in split_str {
        let com = s.split(" ").collect::<Vec<&str>>();
        if com.len() < 2{
            continue;
        }
        if com[0] == "value"{
            let val = com[1].parse::<usize>().unwrap(); 
            let bot = com[5].parse::<usize>().unwrap(); 
            inputs.push((bot,val));
        } else {
            commands.insert(
                com[1].parse::<usize>().unwrap(),
                Command::new(&com),
            );
        }
    }
    for i in &inputs {
        let bot = i.0;
        let val = i.1;
        let v = bots.entry(bot).or_insert(Vec::new());
        v.push(val);

        println!("value {} goes to bot {}", val,bot);

        if v.len() == 2{
            //assert!(commands.get(&bot) != None);
           // assert!(commands.get_mut(&bot) != None);
            commands.get(&bot).unwrap().execute(&mut bots, &commands,&outputs);
        }
    }
//    for b in &bots {
//        println!("{:?}", b);
//    }

    println!("--- --- ---- -----");
    println!("--- --- ---- -----");

//    for b in &bots {
//        println!("{:?}", b);
//    }


}
