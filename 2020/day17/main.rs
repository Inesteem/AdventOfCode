use lib::io::*;

//. -> # = *
//# -> # = #

const INACTIVE : u8 = 0;
const STARTING : u8 = 1;
const ACTIVE : u8 = 2;
const DYING : u8 = 3;

fn print_vec3d(vec : &Vec<Vec<Vec<u8>>>) {
    for depth in 1..vec.len()-1{
        for row in 1..vec[0].len()-1 {
            for col in 1..vec[0][0].len()-1 {
                match vec[depth][row][col] {
                    ACTIVE => print!("#"),
                    INACTIVE => print!("."),
                    STARTING => print!("*"),
                    DYING => print!("-"),
                    _ => assert!(false),
                }
            }
            println!("");
        }
        println!("");
    }
    println!("--------------------");

}

struct Point3D {
    d : usize,
    r : usize,
    c : usize,
}

struct Grid{
    grid : Vec<Vec<Vec<u8>>>,
}

impl Grid {

    fn active_neighbours(&self, point : &Point3D) -> u8 {

        let mut num = 0;
        for depth in point.d-1..=point.d+1 {
            for row in point.r-1..=point.r+1 {
                for col in point.c-1..=point.c+1 {
                    match self.grid[depth][row][col] {
                        ACTIVE => num += 1,
                        DYING => num += 1,
                        _ => (),
                    }
                }
            }
        }
        if self.grid[point.d][point.r][point.c] == ACTIVE {
            num -=1;
        }
        num 
    }

    fn update(&mut self) {
        let mut updates = vec![];
        for depth in 1..self.grid.len()-1{
            for row in 1..self.grid[0].len()-1 {
                for col in 1..self.grid[0][0].len()-1 {
                    let pos = Point3D{d:depth,r:row,c:col};
                    let an = self.active_neighbours(&pos);                   
                    let cube = &mut self.grid[depth][row][col];
                    if *cube == ACTIVE && !(an == 2 || an == 3) {
                        *cube = DYING;
                        updates.push(pos);
                    } else if *cube == INACTIVE && an == 3 {
                        *cube = STARTING;
                        updates.push(pos);
                    }
                    

                }
            }
        }               
        for pos in updates {
            let cube = &mut self.grid[pos.d][pos.r][pos.c];
            if *cube == DYING { *cube = INACTIVE; }
            else if *cube == STARTING{ *cube = ACTIVE;}
            else { assert!(false); }

        }
    }

    fn num_active(&self) -> usize {
        let mut num = 0;
        for depth in 1..self.grid.len()-1{
            for row in 1..self.grid[0].len()-1 {
                for col in 1..self.grid[0][0].len()-1 {
                    let cube = & self.grid[depth][row][col];
                    if *cube == ACTIVE { num += 1; } 
                }
            }
        }
        num 
    }
}



fn main() {
    let input : Vec<Vec<char>>= read_all_from_stdin().split("\n")
                    .filter( |x| x.len() > 0).map(|x| 
                             x.chars().collect()
                    ).collect();

    println!("{:?}", input);
    
    const CYCLES : usize = 7;
    let rows = input.len(); 
    let cols = input[0].len(); 
    let depth = 2*CYCLES + 1;
    let mut grid = vec![vec![vec![INACTIVE;cols + 2*CYCLES];rows + 2*CYCLES];depth];

    for row in CYCLES..CYCLES+rows {            
        for col in CYCLES..CYCLES+cols { 
            grid[CYCLES][row][col] = match input[row-CYCLES][col-CYCLES] {
                '#' => ACTIVE,
                _ => INACTIVE,
    
            };
        }
    }
    let mut grid = Grid{grid : grid};
        //print_vec3d(&grid.grid);
    for _c in 0..CYCLES-1 {
         grid.update();
        println!("cycle {} -> {} active", _c+1,grid.num_active());        
        //print_vec3d(&grid.grid);
    }
}
