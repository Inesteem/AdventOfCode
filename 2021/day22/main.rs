use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use itertools::Itertools;
use std::cmp::min;
use std::cmp::max;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn add_cube(cubes : &mut Vec<Cube>, cube : Cube) {
    if cube.empty() {return;}

    let mut curr_cube = cube;
    let l = cubes.len();
    for i in 0..l {
        if cubes[i].distinct(&curr_cube) { continue; }

        let shared = cubes[i].shared(&curr_cube);
        cubes.push(shared);
    }

    if curr_cube.on {
        cubes.push(curr_cube);
    }

}

#[derive(Debug, Clone)]
struct Range {
    from : isize,
    to : isize,
}

impl Range {
    fn new(s : &str) -> Self {
        let (from,to): (isize,isize) =
                        s.split("..").
                        map(|x| x.parse().unwrap()).
                        collect_tuple().unwrap();

        Range{from : from, to : to+1} // to exclusive
    }
    fn empty(&self) -> bool {
        self.to == self.from
    }
    fn distinct(&self, other : &Range) -> bool {
        self.to <= other.from || self.from >= other.to
    }

    fn shared(&self, other : &Range) -> Range {
        Range{ from : max(self.from, other.from), to : min(self.to, other.to) }
    }
    fn area(&self) -> isize {
        self.to - self.from
    }
}

#[derive(Debug)]
struct Cube {
    x_range : Range,
    y_range : Range,
    z_range : Range,
    on : bool,
}


impl Cube {

    fn shared(&self, other : &Cube) -> Cube {
         Cube{
             y_range : self.y_range.shared(&other.y_range),
             x_range : self.x_range.shared(&other.x_range),
             z_range : self.z_range.shared(&other.z_range),
             on : !self.on 
         }
    }

    fn volumne(&self) -> isize {
        self.y_range.area() * self.x_range.area() * self.z_range.area()
    }

    fn empty(&self) -> bool {
        self.x_range.empty() || self.y_range.empty() || self.z_range.empty()
    }

    fn distinct(&self, other : &Cube) -> bool {
        self.x_range.distinct(&other.x_range) ||
        self.y_range.distinct(&other.y_range) ||
        self.z_range.distinct(&other.z_range)
    }
}

#[derive(Debug)]
struct World {
    //lights : Vec<Vec<Vec<bool>>>, //star1
    lights : Vec<Cube>,
}

impl World {
    fn new() -> Self {
        //World{lights : vec![vec![vec![false;102];102];102]} //star1
        World{lights : vec![]}
    }

    fn parse(&mut self, s : &str) {

        let tmp : (&str, &str)= s.split_whitespace().collect_tuple().unwrap();
        let ranges : Vec<(&str,&str)> = tmp.1.split(",").map(|x|
                                                             x.split("=").collect_tuple().unwrap()
                                                            ).collect::<Vec<(&str, &str)>>();
        let mut x_range = Range{from : 0, to : 0};
        let mut y_range = Range{from : 0, to : 0};
        let mut z_range = Range{from : 0, to : 0};
        let on = tmp.0 == "on";
        for range in ranges {
            let r = Range::new(range.1);
            match range.0 {
                "x" => {
                    x_range = r;
                },
                "y" => {
                    y_range = r;
                }
                "z" => {
                    z_range = r;
                }
                _ => {
                    panic!("parsing error");
                }
            }
        }

        add_cube(&mut self.lights, Cube{x_range : x_range, y_range : y_range, z_range : z_range, on});
        //star1
        //for x in x_range.from..=x_range.to {
        //    if x < -50 || x > 50 { continue; }
        //    for y in y_range.from..=y_range.to {
        //        if y < -50 || y > 50 { continue; }
        //        for z in z_range.from..=z_range.to {
        //            if z < -50 || z > 50 { continue; }
        //            self.lights[(x+50) as usize][(y+50) as usize][(z+50) as usize] = on;
        //        }
        //    }
        //}

    }

    fn lights_on(&self) -> isize {
        let mut cnt = 0;
        for cube in &self.lights {
            if cube.on {
                cnt += cube.volumne();
            } else {
                cnt -= cube.volumne();
            }
        }
        cnt
    }

    //fn lights_on_star1(&self) -> usize {
    //    let mut cnt = 0;
    //    for x in 0..self.lights.len() {
    //        for y in 0..self.lights[0].len() {
    //            for z in 0..self.lights[0][0].len(){
    //                if self.lights[x][y][z] {
    //                    cnt += 1;
    //                }
    //            }
    //        }
    //    }
    //    cnt
    //}

}

fn main() {
    //    std::io::stdin().read_to_string(&mut input).unwrap();
    let files = vec!["test0", "test", "test2", "test3", "data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }


        let lines : Vec<&str>= input.lines().collect();
        let mut world = World::new();

        for line in lines {
            world.parse(line);
        }
        println!("{:?}", world.lights_on());
    }
}
