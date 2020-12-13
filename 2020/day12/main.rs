extern crate lib;
use lib::io::*;
use std::process::exit;


const EAST :  Direction = Direction{x:1,y:0};
const WEST :  Direction = Direction{x:-1,y:0};
const NORTH : Direction = Direction{x:0,y:1};
const SOUTH : Direction = Direction{x:0,y:-1};

#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Action {
    op : char,
    val: i32,
}

impl Action {
    
    fn new(line : &str) -> Self {
        
        Self {
            op : line.chars().nth(0).unwrap(),
            val : line[1..].parse::<i32>().unwrap(),
        }
    }
}

// positive y = north
// positive x = east 
#[derive(Debug)]
struct Ship {
    x : i32,
    y : i32,
    dir : usize,
    DIRS : Vec<Direction>,
}

impl Ship {
    
    fn new() -> Self{
        Self {
            x : 0,
            y : 0,
            dir : 1,
            DIRS : vec![NORTH,EAST,SOUTH,WEST],
        }
    }

    fn move_ship(&mut self, val : i32, dir : usize) {
        self.x += self.DIRS[dir].x * val;
        self.y += self.DIRS[dir].y * val;
    }

    fn do_stuff(&mut self, a : &Action){
        match a.op {
            'F' => self.move_ship(a.val,self.dir),
            'N' => self.move_ship(a.val,0),
            'E' => self.move_ship(a.val,1),
            'S' => self.move_ship(a.val,2),
            'W' => self.move_ship(a.val,3),
            'R' => {
                let num = (a.val/90) as usize;
                self.dir += num ;
                self.dir %= 4;
            },
            'L' => {
                let num = (a.val/90) as usize;
                self.dir += 4-num ;
                self.dir %= 4;
            },

            _ => {
                println!("error while parsing: {}", a.op); 
                exit(-1);
            },
        }
    }

    fn get_manhatten(&self) -> usize {
        (i32::abs(self.x) + i32::abs(self.y)) as usize
    }
}

// STAR 2
//
struct Ship2 {
    x : i32,
    y : i32,
    way_point: WayPoint,
}
struct WayPoint {
    x : i32,
    y : i32,
    DIRS : Vec<Direction>,
    magic : Vec<Box<dyn Fn((&mut i32,&mut i32))>>,
}


impl WayPoint{
    
    fn new() -> Self{
        Self {
            x : 10,
            y : 1,
            DIRS : vec![NORTH,EAST,SOUTH,WEST],
            magic :  vec![
              //  Box::new(move |(x, y)| (y, x)),
              //  Box::new(move |(x, y)| (1 - y, 1 - x)),
            ],
        }
    }

    fn move_waypoint(&mut self, val : i32, dir : usize) {
        self.x += self.DIRS[dir].x * val;
        self.y += self.DIRS[dir].y * val;
    }

    fn do_stuff(&mut self, a : &Action){
        match a.op {
            'N' => self.move_waypoint(a.val,0),
            'E' => self.move_waypoint(a.val,1),
            'S' => self.move_waypoint(a.val,2),
            'W' => self.move_waypoint(a.val,3),
            'R' => {
                let num = ((a.val/90) as usize) % 4;
            },
            'L' => {
                let num = ((a.val/90) as usize) % 4;
            },

            _ => {
                println!("error while parsing: {}", a.op); 
                exit(-1);
            },
        }
    }

}



fn main() {
    let mut ship = Ship::new();
    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break; }

        let a = Action::new(&line[0..line.len()-1]);
        ship.do_stuff(&a);
        //println!("{:?}", a);
        //println!("{:?}\n", ship);
    }
    println!("star1 {}", ship.get_manhatten());
}
