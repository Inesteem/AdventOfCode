use lib::io::*;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug)]
struct Player {
    cards : VecDeque<usize>,
}

impl Player {

    fn play_rec(&mut self, other : &mut Player, set : &mut HashSet<String>) -> i8 {
        //infinite loops prevention
        let mut key = self.to_str();
//        println!("{}", &key);
        key.push_str("-");
        key.push_str(&other.to_str());
        if set.contains(&key) { 
          return 1;   
        }
        set.insert(key);

        if self.cards.len() == 0 { return -1; }
        if other.cards.len() == 0 { return 1; }
        let own = self.cards.pop_front().unwrap();
        let his = other.cards.pop_front().unwrap();
        let mut larger = if own > his {1} else {-1};

        if own <= self.cards.len() && his <= other.cards.len() {
            let mut p0 = Player{cards : VecDeque::new()};
            for i in 0..own {p0.cards.push_back(self.cards[i]);}

            let mut p1 = Player{cards : VecDeque::new()};
            for i in 0..his {p1.cards.push_back(other.cards[i]);}
            
            larger = p0.round_rec(&mut p1);

        }

        if larger == 1 {
            self.cards.push_back(own);
            self.cards.push_back(his);
        } else {
            other.cards.push_back(his);
            other.cards.push_back(own);
        }

        0       
    }
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

    fn to_str(&self) -> String{
        let mut s = String::new();
        for i in 0..self.cards.len() {
            s.push_str(&self.cards[i].to_string());
            s.push_str(".");
        }
        s
    }

    fn round_rec(&mut self, other: &mut Self) -> i8 {
        let mut set = HashSet::new();

        let mut ret = 0;
        while ret == 0 {
            ret = self.play_rec(other, &mut set); 
        }

        ret 
    }


    fn round(&mut self, other: &mut Self) -> usize {
        let mut ret = 0;
        while ret == 0 {
            ret = self.play(other); 
        }
        if ret == 1 {
            return self.cnt();
        }
        other.cnt()
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
    //println!("star1: {}", p0.round(&mut p1));
    

    println!("star2: {}", match p0.round_rec(&mut p1){
        -1 => p1.cnt(),
        _ => p0.cnt(),

    });
}
