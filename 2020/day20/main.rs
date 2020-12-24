//this is the ugliest design i made since years,
//but i just wanted to solve the assignment at some point :(

use lib::io::*;
use std::collections::HashMap;

fn print_vec2D(v : &Vec<Vec<char>>) {
    for row in 0..v.len() {
        for col in 0..v[row].len() {
            print!("{}", v[row][col]);
        }
        println!("");
    }
}

fn count(image : &Vec<Vec<char>>, c : char) -> usize {
    let mut num = 0;
    for row in 0..image.len() {
        for col in 0..image[row].len() {
            if image[row][col] == c { num += 1; }
        }
    }
    num
}

fn find_in(image : &Vec<Vec<char>>, needle : &Vec<Vec<char>>) -> usize{
    let mut num = 0;
    for r in 0..image.len()-needle.len() {
        for c in 0..image[0].len()-needle[0].len() {
            let mut found = true;
            for n_r in 0..needle.len() {
                for n_c in 0..needle[n_r].len() {
                    if needle[n_r][n_c] != ' ' && image[r+n_r][c+n_c] != needle[n_r][n_c] { found = false; break;}
                }
                if !found { break;}
            }
            if found { num += 1};
        }
    }
    num

}

#[derive(Debug)]
struct Similar{
    id : u16,
    side : char,
    direct : bool,
}

#[derive(Debug)]
struct Tile {
    field : Vec<Vec<char>>,
    id : u16,
    similars : Vec<Similar>,
}

impl Tile {
    fn copy_into(&self, image : &mut Vec<Vec<char>>, start_r : usize, start_c : usize) {
         for row in 0..self.field.len()-2 {
            for col in 0..self.field[row].len()-2 {
                assert!(image.len() > start_r + row);
                assert!(image[0].len() > start_c + col);
                image[start_r + row][start_c + col] = self.field[row+1][col+1];
            }
        }
  
    }

    fn print(&self) {
        println!("Tile: {}   {}", self.id, self.similars.len());
        print_vec2D(&self.field);
        for sim in &self.similars {
            println!("{:?}", sim);
        }
    }

    fn rotate(&mut self) {
        let N = self.field[0].len();
        for r in 0..N/2 {
            for c in r..N - r - 1 {
                let tmp =  self.field[r][c];
                self.field[r][c] = self.field[c][N-1-r];
                self.field[c][N-1-r] = self.field[N-1-r][N-1-c];
                self.field[N-1-r][N-1-c] = self.field[N-1-c][r];
                self.field[N-1-c][r] = tmp;
            }
        }
    }
    fn h_flip(&mut self) {

        let l = self.field[0].len()-1;
        for r in 0..self.field.len() {
            for c in 0..self.field[0].len()/2 {
                unsafe{
                    let pa: *mut char  = &mut self.field[r][c];
                    let pb: *mut char  = &mut self.field[r][l-c];
                    std::ptr::swap(pa,pb );
                }
            }
        }
    }

    fn v_flip(&mut self) {

        let l = self.field.len()-1;
        for r in 0..self.field.len()/2 {
            self.field.swap(r,l-r);
        }
    }

    fn check_similar_top(& self, other : & Tile) -> bool{
        //top
        let mut sim_top = true;
        for c in 0..self.field[0].len() {
            if self.field[0][c] != other.field[other.field.len()-1][c] {
                sim_top = false;
            }
        }
        sim_top
    }


    fn check_similar_bottom(& self, other : & Tile) ->bool{
        //top
        let mut sim_bottom = true;
        for c in 0..self.field[0].len() {
            if self.field[self.field.len()-1][c] != other.field[0][c] {
                sim_bottom = false;
            }
        }
        sim_bottom
   }

    fn check_similar_left(& self, other : & Tile) -> bool{
        let mut sim_left = true;
        for r in 0..self.field.len() {
            if self.field[r][0] != other.field[r][self.field[0].len()-1] {
                sim_left = false;
            }
        }
        sim_left
    }
    fn check_similar_right(& self, other : & Tile) -> bool{
        let mut sim_right = true;
        for r in 0..self.field.len() {
            if self.field[r][self.field[0].len()-1] != other.field[r][0] {
                sim_right = false;
            }
        }
        sim_right
    }

    fn check_similar(&mut self, other : &mut Tile) -> char{
        
        if(self.check_similar_top(other))  {
           return 't';
        }
        else if(self.check_similar_bottom(other))  {
           return 'b';
        }
        else if(self.check_similar_left(other))  {
           return 'l';
        }
        else if(self.check_similar_right(other)) {
           return 'r';
        }
        '-' 
    }
//        if sim_top {
//            self.similars.push(Similar{id : other.id, side : 't', direct : true});
//            other.similars.push(Similar{id : self.id, side : 't', direct : true});
//        }
//        if sim_bottom {
//            self.similars.push(Similar{id : other.id, side : 'b', direct : true});
//            other.similars.push(Similar{id : self.id, side : 'b', direct : true});
//        }
}

fn main () {
    
    let mut tiles = vec![];
    let mut idx_map = HashMap::new();
    loop {
        
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break;}
        if line.len() == 1 { continue;}
        if line[..4][..].eq("Tile") {
            tiles.push(Tile{field : vec![], similars : vec![], id : line[5..9][..].parse().unwrap()});
            idx_map.insert(tiles[tiles.len()-1].id, tiles.len()-1);
        } else {
            let l = tiles.len()-1;
            tiles[l].field.push(line.trim().chars().collect());
        }
        
    }
    let mut mul : u64 = 1;    
    for i in 0..tiles.len() {
        unsafe{
            let pa: *mut  Tile = &mut tiles[i];
            for j in i+1..tiles.len() {
                let mut c = '-';
                for k in 0..4 {
                    tiles[i].rotate();
                    //  tiles[i].print();
                    c = (*pa).check_similar(&mut tiles[j]);
                    if c != '-' { break; }
                    tiles[i].h_flip(); 
                    c = (*pa).check_similar(&mut tiles[j]);
                    if c != '-' { break; }
                    tiles[i].h_flip(); 
                    tiles[i].v_flip(); 
                    c = (*pa).check_similar(&mut tiles[j]);
                    if c != '-' { break; }
                    tiles[i].v_flip(); 
                }
                if c != '-' {            
                    (*pa).similars.push(Similar{id : tiles[j].id, side : c, direct : true});
                    let pb: *mut  Tile = &mut tiles[j];
                    (*pb).similars.push(Similar{id : tiles[i].id, side : c, direct : true});
                }


            }
            //    println!("{}: {}", tiles[i].id, num);
        }
        //tiles[i].print();
            println!("{} {}", tiles[i].id, tiles[i].similars.len());
            println!("");
        if tiles[i].similars.len() == 2 {
            mul *= tiles[i].id as u64 ;
        }
    }   
    println!("star1: {}", mul);
    println!("{:?}", idx_map.get(&2473).unwrap());
    let dummy = Tile{ field : vec![], id : 42, similars : vec![]}; 



    //fix orientation and set similar connections
    let mut adapted = vec![false;tiles.len()];
    let mut todo = vec![0 as usize];
    adapted[0] = true;
    let mut top_left = 0;
    unsafe {
        while(todo.len()>0) {
            let i = todo.pop().unwrap();
            let pi: *mut  Tile= &mut tiles[i];
            let mut pos = String::from(""); 
            for sim in &mut (*pi).similars {
                let j = *idx_map.get(&sim.id).unwrap();
                let mut c = '-';
                let pj: *mut  Tile= &mut tiles[j];
                if adapted[j] { 
                    c=(*pi).check_similar(&mut tiles[j]);
                } else {
                    for k in 0..4 {
                        //  tiles[i].print();
                        c = (*pi).check_similar(&mut tiles[j]);
                        if c != '-' { break; }
                        (*pj).h_flip(); 
                        c = (*pi).check_similar(&mut tiles[j]);
                        if c != '-' { break; }
                        (*pj).h_flip(); 
                        (*pj).v_flip(); 
                        c = (*pi).check_similar(&mut tiles[j]);
                        if c != '-' { break; }
                        (*pj).v_flip(); 
                        (*pj).rotate();
                    }
                    todo.push(j);
                }
                if c != '-' {
                    adapted[j] = true;
                    sim.side = c;
                    pos.push(c);
                } else { assert!(false); }
            }
            if pos.eq("tl") || pos.eq("lt") {
                top_left = i;
            }
        }
    }

    //fill image
    let grid_size =  (tiles.len() as f64).sqrt() as usize; 

    let mut image = vec![vec!['-'; (tiles[0].field[0].len()-2) * grid_size]; (tiles[0].field.len()-2)*grid_size];
    
    let mut left = top_left;
    let mut right= top_left;
    let mut start_r = 0;
    let diff = tiles[0].field.len()-2;
    for r in 0..grid_size {
        let mut start_c = 0;
        for c in 0..grid_size {
            tiles[right].copy_into(&mut image, start_r, start_c);
            start_c += diff;
            for sim in &mut tiles[right].similars {
                if sim.side == 'r' { right = *idx_map.get(&sim.id).unwrap(); }
                else if sim.side == 'b' && c == 0 { left = *idx_map.get(&sim.id).unwrap(); }
            }
        }
        start_r += diff;
    }

    println!("");
    let mut image_tile = Tile{id : 0, similars : vec![], field : image};
    
    let sea_monster : Vec<Vec<char>>= read_str_from_file("./sea_monster".to_string()).unwrap().split('\n').map(|x| x.to_string().chars().collect()).collect();
    let test_image: Vec<Vec<char>>= read_str_from_file("./test_image".to_string()).unwrap().split('\n').map(|x| x.to_string().chars().collect()).collect();



    for k in 0..4 {
        //  tiles[i].print();
        println!("{}", find_in(&image_tile.field, &sea_monster));
        image_tile.h_flip(); 
        println!("{}", find_in(&image_tile.field, &sea_monster));
        image_tile.h_flip(); 
        image_tile.v_flip(); 
        println!("{}", find_in(&image_tile.field, &sea_monster));
        image_tile.v_flip(); 
        image_tile.rotate();
    }

//    let num = find_in(&test_image, &sea_monster);
//    println!("star2: {}", count(&test_image, '#') - num * count(&sea_monster,'#'));
}
