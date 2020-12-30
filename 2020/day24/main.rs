use lib::io::*;
use std::collections::HashMap;
    const  E: (i32,i32)  = ( 1, 0);
    const  W : (i32,i32)  = (-1, 0);
    const  SE: (i32,i32)  = ( 0, -1);
    const  NE: (i32,i32)  = ( 1, 1);
    const  SW: (i32,i32)  = (-1, -1);
    const  NW: (i32,i32)  = (0, 1);
    const  BLACK : bool = true;
    const  WHITE : bool = false;


fn main(){
    let mut all_dirs = vec![];

    loop {
        let mut dirs = vec![];
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break;}
        let v : Vec<char>= line.trim().chars().collect();
        
        let mut i = 0;
        while i < v.len() {
            match v[i] {
                's' => {
                    i += 1;
                    match v[i] {
                        'e' => dirs.push(&SE), 
                        'w' => dirs.push(&SW), 
                        _ => assert!(false),
                    }
                },
                'n' => {
                    i += 1;
                    match v[i] {
                        'e' => dirs.push(&NE), 
                        'w' => dirs.push(&NW), 
                        _ => assert!(false),
                    }
                },
                'e' => dirs.push(&E),
                'w' => dirs.push(&W),
                _ => assert!(false),

            }
            i += 1;
        }
        all_dirs.push(dirs);
    }
    let mut tiles = HashMap::new();
    for dirs in all_dirs {
        let mut start = vec![0,0];
        for d in dirs {
            start[0] += d.0;
            start[1] += d.1;
        }        
        let mut id = start[0].to_string();
        id.push('-');
        id.push_str(&start[1].to_string());

        if !tiles.contains_key(&id) {
            tiles.insert(id, BLACK);
        } else{

            let new_val = match *tiles.get(&id).unwrap() {
                WHITE => BLACK,
                BLACK => WHITE,
            };
            tiles.insert(id, new_val);
        } 
    }
    let mut num_b = 0;
    for t in tiles.values() {
        if *t == BLACK { num_b += 1; }
    }
    println!("star1:  {}", num_b);


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut start = [2,3];
        let dirs = vec![&SW, &SW, &SW,
                        &SE ,&SE, &SE,
                        &NE, &NE, &NE,
                        &NW, &NW, &NW];
        for d in dirs {
            start[0] += d.0;
            start[1] += d.1;
        }        
        assert_eq!(start[0],2);
        assert_eq!(start[1],3);
        assert_eq!(1+1,4);
    }

}

}
