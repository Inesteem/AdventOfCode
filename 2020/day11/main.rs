extern crate lib;
use lib::io::*;
use std::cmp::{min,max};
use num::{ToPrimitive,Integer};
use std::mem::swap;
use std::convert::TryFrom;

//use num::cast::AsPrimitive;


const OCCUPIED : char = '#';
const FLOOR: char = '.';
const EMPTY: char = 'L';

#[derive(Debug)]
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
                let nbrs = seat.get_occupied_neighbours2(&self.field); //remove 2 for star1
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
                        if nbrs > 4 { //>3 for star1
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
#[derive(Debug)]
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
#[derive(Debug)] 
pub struct Seat<T> {
    pub x : T,
    pub y : T,
}

impl<T: Copy> Seat<T> {

    fn copy ( other : &Seat<T>) -> Self {
        Self {
            x : other.x,
            y : other.y,
        }
    }

    fn new ( x : T, y : T) -> Self {
        Self {
            x : x,
            y : y,
        }
    }
   }

impl<T: Integer + ToPrimitive + TryFrom<i32> + Copy + std::fmt::Debug> Seat<T> {
//this fuction sucks, wtf
    fn is_seat_in_dir(&self,  dir : &Direction, seat : &mut Seat<T>, seats : &Vec<Vec<char>>) -> bool{
        let x = seat.x.to_i32().unwrap() + dir.x as i32;
        let y = seat.y.to_i32().unwrap() + dir.y as i32;
        if x < 0 || y < 0 || x as usize >= seats[0].len() || y as usize >= seats.len() {
            return false;
        }
       //    print!("{:?} -> ", seat);
        match seats[y as usize][x as usize] {
            OCCUPIED => true,
            EMPTY => false,
            _ => {
                match T::try_from(x) {
                    Ok(x) => seat.x = x,
                    _ => (), 
                }
                match T::try_from(y) {
                    Ok(y) => seat.y = y,
                    _ => (), 
                }
               // seat.x = T::try_from(x);  
               // seat.y = T::try_from(y);
               //println!("{:?}", seat);
                self.is_seat_in_dir(dir,seat,seats)
            }
        }
    }

    fn get_occupied_neighbours2(&self, seats : &Vec<Vec<char>>) -> u8{
        let mut occupied = 0;
        for x in -1..2{
            for y in -1..2 {
                if x == 0 && y == 0 { continue; }
                 let mut seat = Seat::<T>::copy(&self); 
                 let dir = Direction::new(x,y);
                 if self.is_seat_in_dir(&dir, &mut seat, seats){
                    occupied +=1;
                 }
            }
        }
        occupied
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
