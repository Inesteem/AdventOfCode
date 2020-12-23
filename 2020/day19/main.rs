use lib::io::*;

const STAR2 : bool = true;

#[derive(Clone,Debug)]
struct Rule {
    c : char,
    subrules : Vec<Vec<usize>>,
}

impl Rule {
    
    fn match_line(&self, line : &Vec<char>, mut pos : Vec<usize>, rules : &Vec<Rule>) -> Vec<usize>{
        if pos.len() == 0 {return pos;}
        //base case
        if self.c != '-' {
            let mut ret = vec![];
            for p in pos {
                if p >= line.len() { continue; }
                if line[p] == self.c { ret.push(p+1);}
            } 
            return ret;
        }

        let mut rets = vec![];
        for rulelist in &self.subrules {
            let mut positions = pos.clone();

            for rule in rulelist {
                positions = rules[*rule].match_line(line,positions,rules);
                if positions.len() == 0 { break; }
            }
            rets.append(&mut positions); 
            //todo earlz breaking if end found
        }
       rets 
    }

}



fn add_rule(rules : &mut Vec<Rule>, line : &str) {
    let splitted : Vec<&str>= line.split(":").collect();
    let r = splitted[0].parse::<usize>().unwrap();
    if rules.len() <= r { rules.resize(r+1,Rule{c : '-', subrules : vec![]}); }
    
    if splitted[1].contains('"') {
        let vec :Vec<char>= splitted[1].chars().collect();
        rules[r].c = vec[2];
    } else {
        
        let vec : Vec<Vec<usize>> = splitted[1].split("|")
            .map(|x| x.split(' ').filter(|y| y.len() > 0)
                .map(|y| y.parse::<usize>().unwrap())
                .collect())
            .collect();
        rules[r].subrules = vec;
    }
}

fn main () { 
    let mut rules : Vec<Rule> = vec![];

    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 1 { break;}
        add_rule(&mut rules, line.trim());
    }

    if STAR2 {
        add_rule(&mut rules, "8: 42 | 42 8");
        add_rule(&mut rules, "11: 42 31 | 42 11 31");
    }

    let mut sum = 0;
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 1 { continue;}
        if line.len() == 0 { break;}
        let vec : Vec<char> = line.trim().chars().collect();
        let ret = rules[0].match_line(&vec, vec![0], &rules);
        if ret.contains(&(line.len()-1)) {
            sum += 1;
        }
        //println!("{} {:?} {}",  line.trim(), ret, line.len() -1);
    } 
    println!("star2 : {} ", sum);

   // for i in 0..rules.len() {
  //      println!("{} {:?}", i, rules[i]); 
  //  }
}
