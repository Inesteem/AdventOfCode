use lib::io::*;
use std::collections::VecDeque;

#[derive(Debug)]
struct Player {
    cards : VecDeque<usize>,
}

impl Player {

    fn play(&mut self, other : &mut Player) -> i8{
        if self.cards.len() == 0 { return -1; }
        if other.cards.len() == 0 { return 1; }
        let own = self.cards.pop_front().unwrap();
        let his = other.cards.pop_front().unwrap();


        if own > his {
            self.cards.push_back(own);
            self.cards.push_back(his);
        } else {
            other.cards.push_back(his);
            other.cards.push_back(own);
        }

        0       
    }

    fn cnt(& self) -> usize {
        let mut ret = 0;
        for i in 0..self.cards.len() {
            ret += self.cards[i]*(self.cards.len()-i);
        }
        ret

    }

}

fn main() {
    
    let mut p = vec![];
    
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 {break;}
        if line.len() == 1 {continue;}

        if line[..1].eq("P"){
            p.push(Player{cards : VecDeque::new()});
            continue;
        }
        let l = p.len();
        p[l-1].cards.push_back(line.trim().parse().unwrap()); 
    }
    let mut p1 = p.remove(1);
    let mut p0 = p.remove(0);
    let mut ret = 0;
    while ret == 0 {
        ret = p0.play(&mut p1); 
    }
    if ret == 1 {
        println!("star1: {}", p0.cnt());
    } else {
        println!("star1: {}", p1.cnt());
    }
}
