use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;

// first east, first south
fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}



#[derive(PartialEq, Clone, Copy, Debug)]
enum Cucumber{
    NONE,
    EAST,  //>
    SOUTH, //v
}

#[derive(Debug)]
struct Position {
    row : isize,
    col : isize,
}
impl Position {
    fn new(row : usize, col : usize) -> Self {
        Position{row : row as isize, col : col as isize}
    }

    fn up(&self, steps : isize) -> Position {
        Position{row : self.row-(1*steps), col : self.col}
    }

    fn down(&self, steps : isize) -> Position {
        Position{row : self.row+(1*steps), col : self.col}
    }

    fn right(&self, steps : isize) -> Position {
        Position{row : self.row, col : self.col+(1*steps)}
    }

    fn left(&self, steps : isize) -> Position {
        Position{row : self.row, col : self.col-(1*steps)}
    }

}

fn in_field(sea_floor : &Vec<Vec<Cucumber>>, row : usize, col : usize) -> bool {
    if row >= sea_floor.len() || col >= sea_floor[0].len(){ return false; }
    true
}


struct SeaFloor {
    height : usize,
    width : usize,
    floor : Vec<Vec<Cucumber>>,
}

impl SeaFloor {
    fn new(input : String) -> Self {

        let mut floor: Vec<Vec<Cucumber>> = input.lines().
            map(|line| line.chars().
                map(|c|
                    match c {
                        '>' => Cucumber::EAST,
                        'v' => Cucumber::SOUTH,
                        _ => Cucumber::NONE}).collect()).
            collect();
        SeaFloor{ height : floor.len(), width : floor[0].len(), floor : floor }
    }

    fn print(&self) {
        for row in &self.floor {
            for c in row {
                print!("{}", match c {
                    Cucumber::NONE => ".",
                    Cucumber::EAST => ">",
                    Cucumber::SOUTH => "v",
                });
            }
            println!("");
        }
    }

    fn row(&self, row : isize) -> usize {
        if row < 0 { return (self.height as isize + row) as usize; }
        row as usize % self.height
    }

    fn col(&self, col : isize) -> usize {
        if col < 0 { return (self.width as isize + col) as usize; }
        col as usize % self.width
    }

    fn sanitize(&self, pos : Position) -> Position
    {
        Position::new(self.row(pos.row), self.col(pos.col))
    }

    fn set(&mut self, pos : &Position, c : Cucumber) {
        let row = self.row(pos.row);
        let col = self.col(pos.col);
        self.floor[row][col] = c;
    }

    fn at(&self, pos : &Position) -> Cucumber {
        self.floor[self.row(pos.row)][self.col(pos.col)]
    }
}

fn get_moveable_cucumbers(sea_floor : &SeaFloor,
                          easters : &mut Vec<Position>,
                          southers: &mut Vec<Position>)
{

    let mut pos = Position{ row : 0, col : 0};
    for row in 0..sea_floor.height {
        for col in 0..sea_floor.width {
           match sea_floor.at(&pos) {
                Cucumber::EAST => {
                   if sea_floor.at(&pos.right(1)) == Cucumber::NONE {
                       easters.push(Position::new(row, col));
                   }
               },
                Cucumber::SOUTH => {
                    if sea_floor.at(&pos.down(1)) == Cucumber::NONE {
                        southers.push(Position::new(row, col));
                    }
                },
                _ => {},
           }
           pos = pos.right(1);
        }
       pos = pos.down(1);
    }
}

fn add_souther(sea_floor : &mut SeaFloor, pos : &Position, new_southers: &mut Vec<Position>)  {
    let up_pos = pos.up(1);
    if sea_floor.at(&up_pos) == Cucumber::SOUTH {
        new_southers.push(sea_floor.sanitize(up_pos));
    }
}

fn add_easter(sea_floor : &mut SeaFloor, pos : &Position, new_easters: &mut Vec<Position>)  {
    let left_pos = pos.left(1);
        if sea_floor.at(&left_pos) == Cucumber::EAST {
            new_easters.push(sea_floor.sanitize(left_pos));
        }
}
fn move_southers(sea_floor : &mut SeaFloor, southers : &Vec<Position>, new_easters: &mut Vec<Position>, new_southers: &mut Vec<Position>)
{

    for pos in southers {
        let new_pos = pos.down(1);
        if sea_floor.at(&new_pos) != Cucumber::NONE { continue; }

        sea_floor.set(&pos, Cucumber::NONE);
        sea_floor.set(&new_pos, Cucumber::SOUTH);

        if sea_floor.at(&pos.down(2)) == Cucumber::NONE {
            new_southers.push(new_pos);
        }

        add_souther(sea_floor, &pos, new_southers);
        add_easter(sea_floor, &pos, new_easters);
    }
}

fn move_easters(sea_floor : &mut SeaFloor, easters : &Vec<Position>, new_easters: &mut Vec<Position>, new_southers: &mut Vec<Position>){
    for pos in easters {
        let new_pos = pos.right(1);
        if sea_floor.at(&new_pos) != Cucumber::NONE { continue; }

        sea_floor.set(&pos, Cucumber::NONE);
        sea_floor.set(&new_pos, Cucumber::EAST);

        if sea_floor.at(&pos.right(2)) == Cucumber::NONE {
            new_easters.push(new_pos);
        }

        add_souther(sea_floor, &pos, new_southers);
        add_easter(sea_floor, &pos, new_easters);
    }
}

fn main() {

    let files = vec!["data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let mut sea_floor = SeaFloor::new(input);

        let mut easters : Vec<Position> = vec![];
        let mut southers : Vec<Position> = vec![];
        get_moveable_cucumbers(&sea_floor, &mut easters, &mut southers);

        let mut new_easters : Vec<Position> = vec![];
        let mut new_southers : Vec<Position> = vec![];

        for i in 1..1000 {
            move_easters(&mut sea_floor, &easters, &mut new_easters, &mut southers);
            southers.sort_by(|a,b| a.col.cmp(&b.col));
            move_southers(&mut sea_floor, &southers, &mut new_easters, &mut new_southers);

            easters = new_easters;
            easters.sort_by(|a,b| a.row.cmp(&b.row));
            new_easters = vec![];

            southers = new_southers;
            new_southers = vec![];

            if easters.len() == 0 && southers.len() == 0
            {
               println!("\n{}:", i+1);
                //sea_floor.print();
                break;
            }
        }
    }
}

