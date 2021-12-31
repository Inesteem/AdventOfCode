use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::{min,max};
use itertools::Itertools;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}


#[derive(Debug, PartialEq)]
enum Light {
    ON,
    OFF,
    TOGGLE,
}

#[derive(Debug)]
struct Position {
  row : usize,
  col : usize,
}

impl Position {
    fn parse(s : &str) -> Self {
        let (row, col) : (usize, usize) = 
                            s.split(",").map(|x| x.parse().unwrap()).collect_tuple().unwrap();
        Position{ row : row, col : col}
    }
}

#[derive(Debug)]
struct Square {
  from : Position,
  to :   Position,
  light : Light,
}

impl Square {
    fn parse(s : &str) -> Self {
            let parts : Vec<&str> = s.split_whitespace().collect();
            
            let light = match parts[0]  {
                "toggle" =>  Light::TOGGLE,
                _ => match parts[1] {
                    "on" => Light::ON,
                     _ => Light::OFF,
                }

            };

            let (from, to) = match light {
                Light::TOGGLE => (Position::parse(parts[1]), Position::parse(parts[3])),
                _ => (Position::parse(parts[2]), Position::parse(parts[4])),
            };

            Square{ from : from, to : to, light : light}
    }

    fn distinct(&self, other : &Square) -> bool {
        other.from.row > self.to.row || other.to.row < self.from.row || 
        other.from.col > self.to.col || other.to.col < self.from.col 
    }

    fn overlap(&self, other :&Square) -> Self {
        let from = Position{ row : max(self.from.row, other.from.row), 
                             col : max(self.from.col, other.from.col) };
        let to = Position{ row : min(self.to.row, other.to.row), 
                             col : min(self.to.col, other.to.col) };

        let light = match self.light { Light::OFF => Light::ON, _ => Light::OFF };

        Square { from : from, to : to, light : light }
    }

    //fn area(&self) -> usize {
    //    let a = (self.to.row - self.from.row) * (self.to.col - self.from.col);
    //    match self.light {
    //        Light::OFF => -a,
    //        _ => a,
    //    }
    //}

    fn set_star1(&self, field : &mut Vec<Vec<bool>>, row_min : usize, col_min : usize)
    {
        for row in (self.from.row-row_min)..=(self.to.row-row_min)
        {
            for col in (self.from.col-col_min)..=(self.to.col-col_min)
            {
                match self.light {
                    Light::OFF => field[row][col] = false,
                    Light::ON => field[row][col] = true,
                    Light::TOGGLE => field[row][col] = !field[row][col],
                }
            }
        }
    }


    fn set_star2(&self, field : &mut Vec<Vec<usize>>, row_min : usize, col_min : usize)
    {
        for row in (self.from.row-row_min)..=(self.to.row-row_min)
        {
            for col in (self.from.col-col_min)..=(self.to.col-col_min)
            {
                match self.light {
                    Light::OFF => { if field[row][col] > 0 { field[row][col] -= 1; } },
                    Light::ON => field[row][col] += 1,
                    Light::TOGGLE => field[row][col] += 2,
                }
            }
        }
    }

}

//fn add_square(squares : &mut Vec<Square>, mut square : Square) {
//   //wrong 
//    let len = squares.len();
//    for i in 0..len {
//        if square.distinct(&squares[i]) {continue;}
//        squares.push(squares[i].overlap(&square));
//    }
//
//    if square.light == Light::ON {
//        squares.push(square);
//    } else if square.light == Light::TOGGLE {
//        square.light = Light::ON;
//        squares.insert(0,square);
//    }
//
//}

fn main() {
    let files = vec!["data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let lines : Vec<&str>= input.lines().collect();
        
        let mut squares = vec![];
        let mut row_min = 1000000;
        let mut col_min = 1000000;
        let mut col_max = 0;
        let mut row_max = 0;

        for line in lines {
           // add_square(&mut squares, Square::parse(line));
           let square = Square::parse(line);
           row_min = min(row_min, square.from.row);
           row_max = max(row_max, square.to.row);
           col_min = min(col_min, square.from.col);
           col_max = max(col_max, square.to.col);
           squares.push(square);
        }

        let star1 = false;
        let mut lights_on = 0;

        if star1 {
            let mut field = vec![vec![false; col_max-col_min+1]; row_max-row_min+1];

            for square in squares {
                square.set_star1(&mut field, row_min, col_min);
            }

            for row in field {
                for light in row {
                    if light { lights_on += 1; }
                }
            }
        } else {
            let mut field = vec![vec![0; col_max-col_min+1]; row_max-row_min+1];

            for square in squares {
                square.set_star2(&mut field, row_min, col_min);
            }

            for row in field {
                for light in row {
                    lights_on += light; 
                }
            }
        }
            
        
        println!("{}", lights_on);
        //let mut area = 0;
        //for s in &squares {
        //    area += s.area();
        //}
    
        //for square in squares {
        //    println!("{:?}", square);
        //}
        //println!("{}", area);
    }

}
