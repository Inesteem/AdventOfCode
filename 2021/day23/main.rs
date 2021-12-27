use std::cmp::min;
use std::usize;
use std::io::Read;

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
}

const a_goals :  [[Position;2];4] = [
    [Position{row:2,col:3}, Position{row:3,col:3}],
    [Position{row:2,col:5}, Position{row:3,col:5}],
    [Position{row:2,col:7}, Position{row:3,col:7}],
    [Position{row:2,col:9}, Position{row:3,col:9}],
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

        let mut visited = vec![vec![false;13];5];
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
        if self.pos == a_goals[self.a_type as usize][1]
        {
            return true;
        }
        if self.pos == a_goals[self.a_type as usize][0] &&
            field.at(&a_goals[self.a_type as usize][1]) == self.a_type
        {
            return true;
        }

        false
    }

    fn visit(&mut self, pos : &Position)
    {
       self.visited[pos.row as usize][pos.col as usize] = true;
    }

    fn unvisit(&mut self, pos : &Position)
    {
       self.visited[pos.row as usize][pos.col as usize] = false;
    }


    fn go_to(&mut self, field : &mut Field, goal : &Position) -> usize {
        if self.pos == *goal { return 0; }

        let mut energy = usize::max_value()-100000;

        for dir in [Position::up, Position::down, Position::left, Position::right] {
            if self.go(field, &dir) {
                energy = min(energy, self.energy + self.go_to(field, goal));
                self.leave(field, &dir);
            }
        }
        energy
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
                "###B#D#A#C###".chars().map(|c| get_type(c)).collect(),
                "#############".chars().map(|c| get_type(c)).collect(),
        ];
        Self::new(field)
    }
    fn test1() -> Self {
        let field = vec![
                "#############".chars().map(|c| get_type(c)).collect(),
                "#...........#".chars().map(|c| get_type(c)).collect(),
                "###A#B#C#D###".chars().map(|c| get_type(c)).collect(),
                "###A#B#C#D###".chars().map(|c| get_type(c)).collect(),
                "#############".chars().map(|c| get_type(c)).collect(),
        ];
        Self::new(field)
    }
    fn test() -> Self {
        let field = vec![
                "#############".chars().map(|c| get_type(c)).collect(),
                "#...........#".chars().map(|c| get_type(c)).collect(),
                "###B#C#B#D###".chars().map(|c| get_type(c)).collect(),
                "###A#D#C#A###".chars().map(|c| get_type(c)).collect(),
                "#############".chars().map(|c| get_type(c)).collect(),
        ];
        Self::new(field)
    }

    fn set(&mut self, pos : &Position, a_type : AType) {
       self.field[pos.row as usize][pos.col as usize] = a_type;
    }

    fn at(&self, pos : &Position) -> AType {
       self.field[pos.row as usize][pos.col as usize]
    }

    fn is_walkable(&self, pos : &Position) -> bool {
       self.field[pos.row as usize][pos.col as usize] == AType::Walkable
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

fn sim_helper(field : &mut Field, amphipods : &mut Vec<Amphipod>, i : usize,  goals : &[Position], energy : usize, min_energy : &mut usize)
{
    for j in 0..goals.len() {
        if !field.is_walkable(&goals[j]) {continue;}
        //println!("goal: {}", j);
        let goal = goals[j].clone();
        let tmp_energy = amphipods[i].go_to(field, &goal);

        if tmp_energy < usize::max_value()-100000 {
            let pos = amphipods[i].pos.clone();
            amphipods[i].set_pos(goal.clone(), field);
            amphipods[i].walked += 1;
            simulate(field, amphipods, tmp_energy + energy, min_energy);
            amphipods[i].set_pos(pos, field);
            amphipods[i].walked -= 1;
        }
    }
}

fn simulate(field : &mut Field, amphipods : &mut Vec<Amphipod>, energy : usize, min_energy : &mut usize) {
    if energy >= *min_energy { return; }
    //read_char();
    let goals1 : Vec<Position> = (1..11).
        filter(|x| if [3, 5, 7, 9].
        contains(&x) { false } else {true}).
        map(|col| Position{row : 1, col : col}).
        collect();

    for i in 0..amphipods.len() {
        if amphipods[i].in_goal(field) { continue; }
        if amphipods[i].walked == 0 {
            sim_helper(field, amphipods, i, &goals1, energy, min_energy);
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
    println!("{}", *min_energy);
    field.print();
    *min_energy = min(*min_energy, energy);
}

fn main() {

    let mut field = Field::data();

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
     let mut min_energy : usize = usize::max_value();
     simulate(&mut field, &mut amphipods, 0, &mut min_energy);
     println!("{}", min_energy);

}
