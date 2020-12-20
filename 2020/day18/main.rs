use lib::io::*;

fn find_term_end(mut pos : usize, expr : &Vec<char> ) -> i32{
    loop {
        match expr[pos] {
            '(' => {
                let ret = find_term_end(pos + 1, expr);
                assert!(ret >=0);
                pos = (ret as usize) + 1;
            },
            ')' => return pos as i32,
            _ => pos += 1,
        }
        if pos >= expr.len() { break;}
    }
    -1
} 


fn find_term_start(mut pos : usize, expr : &Vec<char> ) -> i32{
    loop {
        match expr[pos] {
            ')' => {
                let ret = find_term_start(pos - 1, expr);
                assert!(ret >=0);
                pos = (ret as usize)-1;
            },
            '(' => return pos as i32,
            _ => pos -= 1,
        }
    }
    -1
} 

fn add_brackets(expr :&mut Vec<char>) {
    let mut pos = 0;
    loop {
        if expr[pos] == '+'{
            let mut bracket_open = pos - 1;
            let mut bracket_close = pos + 2;
            if expr[pos-1] == ')' { bracket_open = find_term_start(pos - 2, expr) as usize;}
            if expr[pos+1] == '(' { bracket_close = find_term_end(pos + 2, expr) as usize;}
            expr.insert(bracket_open as usize,'(');
            expr.insert(1+ (bracket_close as usize),')');
            pos += 2;
        }
        pos += 1;
        if pos == expr.len() { break; } 

    }

}


fn helper2 (mut pos:usize, expr :&Vec<char>) -> (i64,usize) {

    if pos >= expr.len() { return (0,0); };
    let (val,pos2) = do_math2(pos+1,expr);
    if expr[pos] == '(' {
        match expr[pos2] {
            '*' => return (val,pos2-1), 
            ')' => return (val,pos2-1), 
            '+' => return {
                let (val2,pos3) = do_math2(pos2+1,expr);
                return (val+val2,pos3-1);
            },
            _ => assert!(false),
        }
    }
    (val,pos2-1)
}

fn do_math2(mut pos : usize, expr : &Vec<char>) -> (i64,usize) {
    if pos >= expr.len() { return (0,0); };

    let mut op = ' ';
    let mut res = 0;
    
    while pos < expr.len() {
        //println!("{} {} {}", id, expr[pos] , res);
        match expr[pos] {
            '(' => {
                let (zwerg,p) = do_math2(pos+1,expr);
                pos=p;
                match op {
                    '+' => {
                        res+=zwerg;
                    },
                    '*' => {
                        assert!(false);
                        //res*=zwerg;
                    },
                    _ => {
                        res=zwerg;
                    },
                }

            },
            ')' => return (res,pos+1),
            '*' => {

                //if expr[pos+1] == '(' { pos +=1; }
                //let (zwerg,p) = do_math2(pos+1,expr);
                let (zwerg,p) = helper2(pos+1,expr);
                println!("* {}",zwerg);
                res *= zwerg;
                pos=p;
                op = '*';
            },
            '+' => {
                op = '+';
                pos += 1;
            },
            x => {
                let num  =  (x as i64) - ('0' as i64);
                match op {
                    '+' => {
                        res += num;
                        pos += 1;
                    },
                    '*' => {
                        assert!(false);
                    },
                    _ => { 
                        res = num; 
                        pos+=1;
                    },
                }
            },
        }
    }
    (res,pos+1)
}



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
    let mut sum1 : i64 = 0; 
    let mut sum2 : i64 = 0; 
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break; }
        let mut expression : Vec<char> = line.chars().filter(|&c| c != ' ' && c != '\n').collect();
        //println!("{:?}", &expression);
        println!("{}    [{}]", line.trim(), expression.len());
        sum1 += do_math(0,&expression).0;
        println!("{}",do_math(0,&expression).0 );
        
        add_brackets(&mut expression);
        let ret =  do_math(0,&expression).0 ;
        sum2 +=  ret;
        println!("{}\n", ret);
    }
    println!("star1 {}", sum1);
    println!("star2 {}", sum2);//212845023737983 too large 
}
