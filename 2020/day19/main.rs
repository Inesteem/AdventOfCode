use lib::io::*;

#[derive(Clone,Debug)]
struct Rule {
    c : char,
    subrules : Vec<Vec<usize>>,
}

impl Rule {
    
    fn match_line(&self, line : &Vec<char>, pos : usize, rules : &Vec<Rule>) -> i32{
        if pos >= line.len() { return -1;}

        //base case
        if self.c != '-' {
            if line[pos] == self.c { return (pos as i32) + 1;}
            return -1;
        }

        for rulelist in &self.subrules {
            let mut pos_after : i32 = pos as i32;
            for rule in rulelist {
                pos_after = rules[*rule].match_line(line,pos_after as usize,rules);
                if pos_after == -1 { break; }
            } 
            if pos_after != -1 { return pos_after };
        }
        return -1;
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
        println!("{:?}", &vec);
        rules[r].subrules = vec;
    }
}

fn main () { 
    let mut rules : Vec<Rule> = vec![];

    loop {
        println!("{:?}", rules);
        let line = read_line_from_stdin().unwrap();
        if line.len() == 1 { break;}
        println!("{}", &line);
        add_rule(&mut rules, line.trim());
    } 
    let mut sum = 0;
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 1 { continue;}
        if line.len() == 0 { break;}
        let vec : Vec<char> = line.trim().chars().collect();
        if rules[0].match_line(&vec, 0, &rules) + 1 == line.len() as i32 {
            sum += 1;
        }
    } 
    println!("star1 : {} ", sum);
       
}
