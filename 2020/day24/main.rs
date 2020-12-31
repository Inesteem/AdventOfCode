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
const  STAR1 : bool = false;
const SURROUNDING : [&(i32,i32);6]= [&W,&E,&NE,&SE,&NW,&SW];

fn get_pos(id : &str) ->  Vec<i32> {
    let mut v = vec![];
    let s : Vec <&str> = id.split('#').collect();
    v.push(s[0].parse().unwrap());
    v.push(s[1].parse().unwrap());
    v
}


fn get_id(pos: &Vec<i32>) -> String {
    let mut id = pos[0].to_string();
    id.push('#');
    id.push_str(&pos[1].to_string());
    id
}

fn add_surrounding_tiles(tiles : &mut HashMap<String,bool>, start : &Vec<i32>) {
    for d in &SURROUNDING {
        let pos = vec![start[0] + d.0, start[1] + d.1];
        let id = get_id(&pos);
        if !tiles.contains_key(&id) {
            tiles.insert(id, WHITE);
        }
    }
}

fn check_surrounding_tiles(tiles : & HashMap<String,bool>, start : &Vec<i32>) -> i32 {
    let mut b = 0;
    for d in &SURROUNDING {
        let pos = vec![start[0] + d.0, start[1] + d.1];
        let id = get_id(&pos);
        if get_tile_color(tiles, &id) == BLACK { b += 1;}

    }
    b
}

fn get_tile_color(tiles : &HashMap<String,bool>, id : &str) -> bool {

    if !tiles.contains_key(id) {
        return WHITE;
    } else {
        return *tiles.get(&*id).unwrap();
    } 
}

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
        let id = get_id(&start); 

        let new_val = match get_tile_color(&tiles, &id) {
            WHITE => BLACK,
            BLACK => WHITE,
        };
        tiles.insert(id, new_val);
        if(new_val == BLACK) {
            add_surrounding_tiles(&mut tiles, &start);
        }
    }


    for _d in 0..100 {

        let mut to_flip = vec![];
        for id in tiles.keys() {
            let pos = get_pos(id);
            let num_b = check_surrounding_tiles(&tiles, &pos);
            match *tiles.get(id).unwrap(){
                BLACK => {
                    if num_b == 0 || num_b > 2 {
                        to_flip.push(id.to_string());
                    }
                },
                WHITE => {
                    if num_b == 2 {
                        to_flip.push(id.to_string());
                    }
                },
            }
        }    
        for id in to_flip {
           let new_val = !tiles[&id];
            *tiles.get_mut(&id).unwrap() = new_val;
            if new_val == BLACK {
                let pos = get_pos(&id);
                add_surrounding_tiles(&mut tiles, &pos);
            }
        }
    }

    let mut num_b = 0;
    for t in tiles.values() {
        if *t == BLACK { num_b += 1; }
    }
    println!("star1:  {}", num_b);

}



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
    }

}


