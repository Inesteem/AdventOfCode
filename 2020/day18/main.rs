use lib::io::*;

fn helper(pos : usize, expr : &Vec<char>) -> (i64,usize) {
    match expr[pos] {
        '(' => return do_math(pos+1, expr),
        ')' => assert!(false),
        '*' => assert!(false),
        '+' =>assert!(false),
        x => return ((x as i64) - ('0' as i64),pos+1),
    }
    (0,0)
}

    
fn do_math(mut pos : usize, expr : &Vec<char>) -> (i64,usize) {
    if pos >= expr.len() { return (0,0); };
    
    let mut res : i64;
    let (r,p) = helper(pos,expr);
    res = r; pos = p;
    while pos < expr.len() {
        match expr[pos] {
            '(' => assert!(false),
            ')' => return (res,pos+1),
            '*' => {
                let (zwerg,p) = helper(pos+1,expr);
                res *= zwerg;
                pos = p;
            },
            '+' => {
                let (zwerg,p) = helper(pos+1,expr);
                res += zwerg;
                pos = p;
            },
            _ => assert!(false),
        }
    }
    (res,pos+1)
}


fn main() {
    let mut sum : i64 = 0; 
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break; }
        let expression : Vec<char> = line.chars().filter(|&c| c != ' ' && c != '\n').collect();
        println!("{:?}", &expression);
        sum += do_math(0,&expression).0 as i64;
    }
    println!("{}", sum);
}
