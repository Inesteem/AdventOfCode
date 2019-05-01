use std::env;
use std::cmp;
use std::ops;


#[derive(Debug)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn get_hamming_dist(&self, other: &Position) -> u64 {
        let dist_x = (self.x as i64 - other.x as i64).abs();
        let dist_y = (self.y as i64 - other.y as i64).abs();
        (dist_x + dist_y) as u64 
    }
}

impl std::cmp::PartialEq for Position {

    fn eq(&self, _rhs: &Position) -> bool {
        (self.x == _rhs.x) && (self.y == _rhs.y)
    }
}
impl std::cmp::Eq for Position {}

impl std::ops::Add<(i64,i64)> for &Position {
    type Output = Position;

    fn add(self, _rhs: (i64,i64)) -> Position{
        Position {
            x: (self.x as i64 +_rhs.0) as u64, 
            y: (self.y as i64 +_rhs.1) as u64, 
        }
    }
}


fn numberOfSetBits(num: u64) -> u64{
    let mut x = num;
    let mut cnt = 0;
    while x != 0 {
        cnt += x & 0x1;
        x >>= 1;
    }
    cnt
}

fn is_free(x: u64, y: u64, magic: &u64)-> bool {
    let basic: u64 = x*x + 3*x + 2*x*y +y +y*y + *magic;
    if (numberOfSetBits(basic) % 2) == 0 {
        return true 
    }
    false
}


fn crawl(field: &mut Vec<Vec<u64>>, pos: &Position, dst: &Position, way: u64, min: &mut u64, magic: &u64) {
    if pos == dst {

        *min = std::cmp::min(*min, way);
         return;
    }

    if way >= *min {
        return;
    }

    if !is_free(pos.x, pos.y, magic) {
        return;
    }

    if pos.y as usize == field.len() {
        let vec = vec![std::u64::MAX; field[pos.y as usize - 1].len()];
        field.push(vec);
    }

    if pos.x as usize >= field[pos.y as usize ].len() {
       field[pos.y as usize].resize(pos.x as usize +1, std::u64::MAX); 
    }

    if field[pos.y as usize][pos.x as usize] <= way {
        return;
    }


    field[pos.y as usize][pos.x as usize] = way;

    let right = pos + (1,0);
    let down = pos + (0,1);

    let mut next_pos = vec![
        (right.get_hamming_dist(dst), right),
        (down.get_hamming_dist(dst), down),
    ];
    if pos.x > 0 {
        let left = pos + (-1,0);
        next_pos.push((left.get_hamming_dist(dst), left));
    }
    if pos.y > 0 {
        let up = pos + (0,-1);
        next_pos.push((up.get_hamming_dist(dst), up));
    }
//    next_pos.sort_by(|a,b| a.0.cmp(&b.0).reverse());
    next_pos.sort_by(|a,b| a.0.cmp(&b.0));


    for (d,p) in next_pos {
        crawl(field, &p, dst, way + 1, min, magic);
    }

    

}


fn crawl_50(field: &mut Vec<Vec<bool>>, pos: &Position, way: u64) {
    if way == 50 {
        return;
    }

    if !is_free(pos.x, pos.y, magic) {
        return;
    }

    if field[pos.y as usize][pos.x as usize] {
        return;
    }

    field[pos.y as usize][pos.x as usize] = true;

    let right = pos + (1,0);
    let down = pos + (0,1);

    let mut next_pos = vec![
        (right.get_hamming_dist(dst), right),
        (down.get_hamming_dist(dst), down),
    ];
    if pos.x > 0 {
        let left = pos + (-1,0);
        next_pos.push((left.get_hamming_dist(dst), left));
    }
    if pos.y > 0 {
        let up = pos + (0,-1);
        next_pos.push((up.get_hamming_dist(dst), up));
    }
    next_pos.sort_by(|a,b| a.0.cmp(&b.0));


    for (d,p) in next_pos {
        crawl(field, &p, way + 1);
    }

    

}





fn main() {
    let args: Vec<String> = env::args().collect();

    let magic = (&args[1]).parse::<u64>().unwrap(); 

    println!("magic num: {}",magic);

    /*
    for y in 0..10 {
        for x in 0..10 {
            if is_free(x,y,magic) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    } */

    let dst = Position {x:31,y:39};
//    let dst = Position {x:7,y:4};

    let mut field = vec![vec![std::u64::MAX; dst.x as usize + 1]; dst.y as usize + 1];

    let mut min = std::u64::MAX;

    let pos = Position {x:1 , y:1};

    crawl(&mut field, &pos, &dst, 0, &mut min, &magic);



    for y in 0..field.len() {
        for x in 0..field[y].len() {
            if y as u64 == dst.y && x as u64 == dst.x {
                print!("X");
            }
            else if field[y][x] != std::u64::MAX {
                print!("o");
                assert!(is_free(x as u64,y as u64, &magic));
            }
            else if is_free(x as u64,y as u64, &magic) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    } 

    println!("\nmin: {}", min);


    let mut field2 = vec![vec![false; 52]; 52];


    crawl_50(field2, pos, 0);

    let mut cnt = 0;

    for y in 0..field2.len() {
        for x in 0..field2[y].len() {
            if field[y][x] {
                print!("o");
                assert!(is_free(x as u64,y as u64, &magic));
                cnt += 1;
            }
            else if is_free(x as u64,y as u64, &magic) {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    } 


    
    println!("\ncnt: {}", cnt);

    // to high: 104, 92
}
