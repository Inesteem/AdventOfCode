extern crate lib;
use lib::io::*;
use std::cmp::{min,max};
use num::{ToPrimitive,Integer};
use std::mem::swap;

//use num::cast::AsPrimitive;


const OCCUPIED : char = '#';
const FLOOR: char = '.';
const EMPTY: char = 'L';

pub struct Field<'a>{
    pub field : &'a mut Vec<Vec<char>>,
    pub new_field : &'a mut Vec<Vec<char>>,
}
impl <'a> Field<'a> {

    fn new(of : &'a mut Vec<Vec<char>>, nf: &'a mut Vec<Vec<char>>) -> Field<'a> {
        Field {
            field : of,
            new_field : nf,
        }
    }

    fn update(&mut self) -> bool {
        let mut changed = false; 
        let mut seat = Seat::<usize>::new(0,0);
        for y in 0..self.field.len(){
            seat.y=y;
            for x in 0..self.field[y].len() {
                seat.x=x;
                let nbrs = seat.get_occupied_neighbours(&self.field);
                self.new_field[y][x] = match self.field[y][x] {
                    EMPTY => {
                        if nbrs == 0 { 
                            changed = true;
                            OCCUPIED
                        } else {
                            EMPTY
                        }
                    },
                    OCCUPIED => {
                        if nbrs > 3 {
                            changed = true;
                            EMPTY
                        } else {
                            OCCUPIED
                        }
                    },
                    _=> FLOOR,

                };
            }
        }     
        swap(&mut self.new_field, &mut self.field);
        changed 
    }
    
    fn occupied_seats(&self)-> u32 {
        let mut occupied = 0;
        for y in 0..self.field.len(){
            for x in 0..self.field[y].len() {
                if self.field[y][x] == OCCUPIED {
                    occupied += 1;
                }
            }
        }
        occupied
    }
    
       
}

pub struct Direction {
    x : i8,
    y : i8,
}

impl Direction {

    fn new ( x : i8, y : i8) -> Self {
        Self {
            x : x,
            y : y,
        }
    }
}
   
pub struct Seat<T> {
    pub x : T,
    pub y : T,
}

impl<T> Seat<T> {

    fn new ( x : T, y : T) -> Self {
        Self {
            x : x,
            y : y,
        }
    }
   }

impl<T: Integer + ToPrimitive> Seat<T> {
    fn is_seat_in_dir(&self,  dir : &Direction, seat : &mut Seat<T>, seats : &Vec<Vec<char>>) -> bool{
        let x = seat.x.to_i32().unwrap() + dir.x as i32;
        let y = seat.y.to_i32().unwrap() + dir.y as i32;
        if x < 0 || y < 0 || x >= seats[0].len() || y >= seats.len() {
            return false;
        }
        match seats[y as usize][x as usize] {
            OCCUPIED => true,
            EMPTY => false,
            _ => {
                seat.x = x;
                seat.y = y;
                is_seat_in_dir(dir,seat,seats)
            }
        }
    }

    fn get_occupied_neighbours(&self, seats : &Vec<Vec<char>>) -> u8{
        let mut occupied : u8 = 0;
        let x = self.x.to_i32().unwrap();
        let y = self.y.to_i32().unwrap();
        let x_start = max(x - 1, 0);
        let y_start = max(y - 1, 0);
        let x_end = min(x + 2, seats[0].len() as i32 );
        let y_end = min(y + 2, seats.len() as i32 );
        //print!("{} {} - {} {} - {} {} -> ", x,y,x_start,x_end,y_start,y_end);
        for y in y_start as usize..y_end as usize {
            for x in x_start as usize..x_end as usize{
               if seats[y][x] == OCCUPIED {
                    occupied += 1;
               }
            }
        }
        //println!(" {} {} ", occupied, seats[y as usize][x as usize]);
        if seats[y as usize][x as usize] == OCCUPIED {
            occupied -=1;
        }
        occupied
    }
    
}

//fn update(seat 

fn main() {

    let mut seats : Vec<Vec<char>>= Vec::new();

    loop {
        let line = read_line_from_stdin().unwrap();
        if line.len() == 0 { break;}
        
        seats.push(line[0..line.len()-1].chars().collect());
    }
    let mut new_seats = vec![vec![FLOOR; seats[0].len()]; seats.len()];



    let mut field = Field::new(&mut seats,&mut new_seats);
    while field.update() {
  //      println!("{:?}", field.field);
    }
//    println!("\n{:?}", field.field);

    println!("star1: {}", field.occupied_seats());
}
