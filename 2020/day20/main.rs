use lib::io::*;

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

    fn print(&self) {
        println!("Tile: {}   {}", self.id, self.similars.len());
        for row in &self.field {
            for c in row {
                print!("{}", c);
            }
            println!("");
        }
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
            if self.field[0][c] != other.field[0][c] {
                sim_top = false;
            }
        }
        sim_top
    }


    fn check_similar_bottom(& self, other : & Tile) ->bool{
        //top
        let mut sim_bottom = true;
        for c in 0..self.field[0].len() {
            if self.field[self.field.len()-1][c] != other.field[other.field.len()-1][c] {
                sim_bottom = false;
            }
        }
        sim_bottom
   }

    fn check_similar_left(& self, other : & Tile) -> bool{
        let mut sim_left = true;
        for r in 0..self.field.len() {
            if self.field[r][0] != other.field[r][0] {
                sim_left = false;
            }
        }
        sim_left
    }
    fn check_similar_right(& self, other : & Tile) -> bool{
        let mut sim_right = true;
        for r in 0..self.field.len() {
            if self.field[r][self.field[0].len()-1] != other.field[r][self.field[0].len()-1] {
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
    loop {
        
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break;}
        if line.len() == 1 { continue;}
        if line[..4][..].eq("Tile") {
            tiles.push(Tile{field : vec![], similars : vec![], id : line[5..9][..].parse().unwrap()});
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

    let grid_size =  (tiles.len() as f64).sqrt() as usize; 


    


}
