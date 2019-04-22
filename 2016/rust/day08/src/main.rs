use std::env;
use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashMap;
const HEIGHT: usize = 6;
const WIDTH: usize = 50;
//const HEIGHT: usize =3;
//const WIDTH: usize = 7;


fn print(display: &[[bool; WIDTH]; HEIGHT]){
    let mut cnt = 0;
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            if display[r][c] == true {
                print!("#");
                cnt+=1;
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    println!("{}", cnt);
}


fn rect(display: &mut [[bool; WIDTH]; HEIGHT], v: Vec<&str>){
    println!("rect");
    let args = v[1].split("x").collect::<Vec<&str>>();
    print!("{:?} -> ", args);
    let cols = args[0].parse::<usize>().unwrap(); 
    let rows = args[1].parse::<usize>().unwrap(); 
    println!("{} {}", cols, rows);
    assert!(cols <=  WIDTH && rows <= HEIGHT);

    for r in 0..rows {
        for c in 0..cols{
            display[r][c] = true;
        }
    }    

    
}
fn col(display: &mut [[bool; WIDTH]; HEIGHT], v: Vec<&str>){
    let args = v[2].split("=").collect::<Vec<&str>>();
    let c = args[1].parse::<usize>().unwrap(); 
    let steps = (v[4].parse::<usize>().unwrap()) % HEIGHT; 
    println!("col: {} by {}", c, steps);
    assert!(c <= WIDTH);

    let mut tmp = [false; HEIGHT];

    for r in 0..HEIGHT{
        tmp[r] = display[(r + HEIGHT - steps) % HEIGHT][c];
    }     
    for r in 0..HEIGHT{
        display[r][c] = tmp[r];
    }     
    

}
fn row(display: &mut [[bool; WIDTH]; HEIGHT], v: Vec<&str>){
    let args = v[2].split("=").collect::<Vec<&str>>();
    let r = args[1].parse::<usize>().unwrap(); 
    let steps = (v[4].parse::<usize>().unwrap()) % WIDTH; 
    println!("row: {} by {}", r, steps);
    assert!(r <= WIDTH);

    
    let mut tmp = [false; WIDTH];     

    for c in 0..WIDTH{
        tmp[c] = display[r][(c + WIDTH - steps ) % WIDTH];
    }     
    for c in 0..WIDTH{
        display[r][c] = tmp[c];
    }     

}

fn main() {
//    let mut commands = HashMap::new();
//
//    let f: &Fn() -> bool= &rect;
//
//    commands.insert(
//        "rect".to_string(),
//        f,
//    );


    let mut display = [[false; WIDTH]; HEIGHT];     

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let split_str = contents.split("\n");
    for s in split_str {
        let com = s.split(" ").collect::<Vec<&str>>();
        if com.len() < 2{
            continue;
        }
        
        if com[0] == "rect"{
            rect(&mut display, com);
        }
        else if com[1] == "column"{
            col(&mut display, com);
        }  
        else if com[1] == "row"{
            row(&mut display, com);
        }  
        else {
            assert!(false);
        }
    }
    print(&display);
}
