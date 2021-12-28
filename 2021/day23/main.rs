use std::cmp::{min, max};
use std::process;
use std::usize;
use std::io::Read;

// Probably there is a nice dp solution. I was too stupid for this shit, but hard pruning also
// works fine. Needs significantly more time.

fn read_char() -> i32{
    let input: Option<i32> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);
    input.unwrap()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AType {
    Amber=0,
    Bronze=1,
    Copper=2,
    Desert=3,
    Walkable,
    Wall,
    Mark1,
    Mark2,
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    row : isize,
    col : isize,
}

impl Position {

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

    //11 is to much.you probably only need to check 7 places
    //because no amphipod ever stands above homes and no amphipod in 1 and 11 is in the way
    fn idx(&self) -> usize { //max 11 + num_amph*4
        if self.row == 1 {return (self.col - 1) as usize;}
        (11 + (4*(self.row-2)) + (self.col - 3) / 2) as usize
    }
}

const a_goals : [[Position;4];4] = [
    [Position{row:5,col:3}, Position{row:4,col:3}, Position{row:3,col:3}, Position{row:2,col:3}],
    [Position{row:5,col:5}, Position{row:4,col:5}, Position{row:3,col:5}, Position{row:2,col:5}],
    [Position{row:5,col:7}, Position{row:4,col:7}, Position{row:3,col:7}, Position{row:2,col:7}],
    [Position{row:5,col:9}, Position{row:4,col:9}, Position{row:3,col:9}, Position{row:2,col:9}],
];
//const a_goals : [[Position;2];4] = [
//    [Position{row:3,col:3}, Position{row:2,col:3}],
//    [Position{row:3,col:5}, Position{row:2,col:5}],
//    [Position{row:3,col:7}, Position{row:2,col:7}],
//    [Position{row:3,col:9}, Position{row:2,col:9}],
//];
const goals_left : [Position;7] = [
    Position{row:1,col:1}, Position{row:1,col:2},
    Position{row:1,col:4},
    Position{row:1,col:6}, Position{row:1,col:8},
    Position{row:1,col:10}, Position{row:1,col:11},
];

const goals_right : [Position;7] = [
    Position{row:1,col:10}, Position{row:1,col:11},
    Position{row:1,col:8},
    Position{row:1,col:4}, Position{row:1,col:6},
    Position{row:1,col:1}, Position{row:1,col:2},
];


#[derive(Debug)]
struct Amphipod {
    pos : Position,
    energy : usize,
    walked : usize,
    visited : Vec<Vec<bool>>,
    a_type : AType,
}


impl Amphipod {
    fn new(pos : Position, energy : usize, a_type : AType) -> Self {

        let mut visited = vec![vec![false;13];7];
        Amphipod{pos : pos, energy: energy, walked : 0, visited : visited,a_type : a_type}
    }

    fn amber(pos : Position) -> Self {
        Self::new(pos, 1, AType::Amber)
    }
    fn bronze(pos : Position) -> Self {
        Self::new(pos, 10, AType::Bronze)
    }
    fn copper(pos : Position) -> Self {
        Self::new(pos, 100, AType::Copper)
    }
    fn desert(pos : Position) -> Self {
        Self::new(pos, 1000, AType::Desert)
    }

    //returning always possible
    fn leave(&mut self, field : &mut Field, in_dir : &dyn Fn(&Position, isize) -> Position) {
        let nextPos = in_dir(&self.pos, -1);
        self.visited[self.pos.row as usize][self.pos.col as usize] = false;

        self.set_pos(nextPos, field);
    }

    fn go(&mut self, field : &mut Field, in_dir : &dyn Fn(&Position, isize) -> Position) -> bool {
        let nextPos = in_dir(&self.pos, 1);
        if !field.is_walkable(&nextPos) { return false; }

        if self.visited[nextPos.row as usize][nextPos.col as usize] { return false;}
        self.visited[nextPos.row as usize][nextPos.col as usize] = true;

        self.set_pos(nextPos, field);
        true
    }

    fn set_pos(&mut self, nextPos : Position, field : &mut Field) {
        field.set(&self.pos, AType::Walkable);
        self.pos = nextPos;
        field.set(&self.pos, self.a_type);
    }


    fn in_goal(&self, field : &Field) -> bool
    {
        if self.pos.col != a_goals[self.a_type as usize][0].col
        {
            return false;
        }
        for p in &a_goals[self.a_type as usize]
        {
            if !field.is_walkable(p) && field.at(p) != self.a_type { return false;}
        }

       true 
    }



}

fn get_char(t : AType) -> char{
    match t {
        AType::Amber    => 'A',
        AType::Bronze   => 'B',
        AType::Copper   => 'C',
        AType::Desert   => 'D',
        AType::Wall     => '#',
        AType::Walkable => '.',
        AType::Mark1 => '*',
        AType::Mark2 => '+',
    }
}
fn get_type(c : char) -> AType{
    match c {
        'A' => AType::Amber,
        'B' => AType::Bronze,
        'C' => AType::Copper,
        'D' => AType::Desert,
        '#' => AType::Wall,
        '.' => AType::Walkable,
        _ => panic!("not valid")
    }
}

#[derive(Debug)]
struct Field {
    field : Vec<Vec<AType>>,
}


impl Field {

    fn new(field : Vec<Vec<AType>>) -> Self {

        Field {field : field}
    }
    fn data() -> Self {
        let field = vec![
            "#############".chars().map(|c| get_type(c)).collect(),
            "#...........#".chars().map(|c| get_type(c)).collect(),
            "###D#B#A#C###".chars().map(|c| get_type(c)).collect(),
            "###D#C#B#A###".chars().map(|c| get_type(c)).collect(),//star2
            "###D#B#A#C###".chars().map(|c| get_type(c)).collect(),//star2
            "###B#D#A#C###".chars().map(|c| get_type(c)).collect(),
            "#############".chars().map(|c| get_type(c)).collect(),
        ];
            Self::new(field)
    }
    fn test() -> Self {
        let field = vec![
            "#############".chars().map(|c| get_type(c)).collect(),
            "#...........#".chars().map(|c| get_type(c)).collect(),
            "###B#C#B#D###".chars().map(|c| get_type(c)).collect(),
            "###D#C#B#A###".chars().map(|c| get_type(c)).collect(),//star2
            "###D#B#A#C###".chars().map(|c| get_type(c)).collect(),//star2
            "###A#D#C#A###".chars().map(|c| get_type(c)).collect(),
            "#############".chars().map(|c| get_type(c)).collect(),
        ];
        Self::new(field)
    }

    fn set(&mut self, pos : &Position, a_type : AType) {
        self.field[pos.row as usize][pos.col as usize] = a_type;
    }

    fn mark(&mut self, pos : &Position, a_type : AType) -> AType {
        let tmp = self.at(pos);
        self.set(pos, a_type);
        tmp
    }

    fn at(&self, pos : &Position) -> AType {
        self.field[pos.row as usize][pos.col as usize]
    }

    fn is_walkable(&self, pos : &Position) -> bool {
        self.field[pos.row as usize][pos.col as usize] == AType::Walkable
    }

    fn is_walkable2(&self, row : usize, col : usize) -> bool {
        self.field[row][col] == AType::Walkable
    }

    fn print(&self) {
        for row in &self.field {
            for col in row {
                print!("{}", get_char(*col));
            }
            println!();
        }
    }
}

fn go_to(field : &mut Field, curr : &Position, goal : &Position) -> (bool, usize) {
    let start_row = min(curr.row, goal.row) as usize;
    let end_row = max(curr.row, goal.row) as usize;

    let start_col = min(curr.col, goal.col) as usize;
    let end_col = max(curr.col, goal.col) as usize;

    if(curr.row > goal.row) {
        for row in start_row+1..end_row {
            if !field.is_walkable2(row as usize, curr.col as usize) {
                return (false, usize::max_value()); 
            }
        }

        for col in start_col..=end_col {
            if !field.is_walkable2(goal.row as usize, col as usize) { 
                return (true, usize::max_value()); 
            }
        }
    } else {
        for col in start_col+1..end_col {
            if col as isize == curr.col { continue; }
            if !field.is_walkable2(curr.row as usize, col as usize) { 
                return (false, usize::max_value()); 
            }
        }

        for row in start_row+1..=end_row {
            if !field.is_walkable2(row as usize, goal.col as usize) {
                return (true, usize::max_value()); 
            }
        }
    }

    return (true, (end_row - start_row) + (end_col - start_col));

    
}

fn sim_helper(field : &mut Field, amphipods : &mut Vec<Amphipod>, i : usize,  goals : &[Position], energy : usize, min_energy : &mut usize)
{
    for j in 0..goals.len() {
        if !field.is_walkable(&goals[j]) {
            if amphipods[i].walked == 1 && field.at(&goals[j]) != amphipods[i].a_type { return; }
            continue;
        }
        //println!("goal: {}", j);

        let (cont, steps) = go_to(field, &amphipods[i].pos, &goals[j]);
    //let tmp1 = field.mark(&amphipods[i].pos, AType::Mark1);
    //let tmp2 = field.mark(&goals[j], AType::Mark2);
    //field.print();
    //field.set(&amphipods[i].pos, tmp1);
    //field.set(&goals[j], tmp2);
    //println!("{:?} -> {:?} {} {}", amphipods[i].pos, goals[j], cont, steps);
    //read_char();

        if steps < usize::max_value() {
            let pos = amphipods[i].pos.clone();
            amphipods[i].set_pos(goals[j].clone(), field);
            amphipods[i].walked += 1;
            simulate(field, amphipods, steps*amphipods[i].energy + energy, min_energy);
            amphipods[i].set_pos(pos, field);
            amphipods[i].walked -= 1;
            if amphipods[i].walked == 1 { return; } // going fully down is always the best choice
        } else if !cont { return; }
    }
}

fn simulate(field : &mut Field, amphipods : &mut Vec<Amphipod>, energy : usize, min_energy : &mut usize) {
    if energy >= *min_energy { return; }
    //let goals1 : Vec<Position> = (1..11).
    //    filter(|x| if [3, 5, 7, 9].
    //           contains(&x) { false } else {true}).
    //    map(|col| Position{row : 1, col : col}).
    //    collect();

    for i in 0..amphipods.len() {
        if amphipods[i].in_goal(field) { continue; }
        if amphipods[i].walked == 0 {
            //if amphipods[i].pos.col < 6 {
            if a_goals[amphipods[i].a_type as usize][0].col < 6 {
                sim_helper(field, amphipods, i, &goals_left, energy, min_energy);
            } else {
                sim_helper(field, amphipods, i, &goals_right, energy, min_energy);
            }
        } else if amphipods[i].walked == 1 {
            sim_helper(field, amphipods, i, &a_goals[amphipods[i].a_type as usize], energy, min_energy);
        } else {
            //println!("{}: {:?}", i, amphipods[i]);
            //panic!("should not happen");
        }
    }

    for i in 0..amphipods.len() {
        if !amphipods[i].in_goal(field) { return; }
    }
    println!("{}\n", *min_energy);
    *min_energy = min(*min_energy, energy);
}

fn main() {

    let mut field = Field::test();

    let mut amphipods = vec![];
    for row in 0..field.field.len(){
        for col in 0..field.field[row].len() {
            let pos = Position{row : row as isize, col : col as isize};
            match field.field[row as usize][col as usize] {
                AType::Amber  => amphipods.push(Amphipod::amber(pos)),
                AType::Bronze => amphipods.push(Amphipod::bronze(pos)),
                AType::Copper => amphipods.push(Amphipod::copper(pos)),
                AType::Desert => amphipods.push(Amphipod::desert(pos)),
                _ => {},
            }
        }
    }
    amphipods.sort_by(|a,b| b.energy.cmp(&a.energy));

    let mut min_energy : usize = usize::max_value();
    simulate(&mut field, &mut amphipods, 0, &mut min_energy);
    println!("{}", min_energy);

}
