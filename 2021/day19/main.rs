use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashSet;
use std::fmt;
use std::cmp::max;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}
//fn read_in_one_char() {
//    let input: Option<i32> = std::io::stdin()
//        .bytes()
//        .next()
//        .and_then(|result| result.ok())
//        .map(|byte| byte as i32);
//}


type Mutation =(Transformation,Transformation,Transformation,Transformation,Translation);
#[derive(Debug,Clone,Hash)]
pub struct Beacon
{
    x : isize,
    y : isize,
    z : isize,
}

impl fmt::Display for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "< {} {} {} >", self.x, self.y, self.z)
    }
}


impl PartialEq for Beacon {
    fn eq(&self, other : &Self) -> bool {
        return self.x == other.x &&
            self.y == other.y &&
            self.z == other.z
    }
}
impl Eq for Beacon {}

#[derive(Debug,PartialEq,EnumIter,Clone,Hash,Eq)]
pub enum Transformation {
    Same,
    XRot,
    YRot,
    ZRot,
}

pub fn rotate_x(beacon : &mut Beacon) {
    let tmp  = beacon.y;
    beacon.y = beacon.z * -1;
    beacon.z = tmp;
}
pub fn rotate_y(beacon : &mut Beacon) {
    let tmp  = beacon.z;
    beacon.z = beacon.x * -1;
    beacon.x = tmp;
}
pub fn rotate_z(beacon : &mut Beacon) {
    let tmp  = beacon.y;
    beacon.y = beacon.x * -1;
    beacon.x = tmp;
}

pub fn applyTrans(beacon : &mut Beacon, trafo : &Transformation) {
    match trafo {
        Transformation::XRot => rotate_x(beacon),
        Transformation::YRot => rotate_y(beacon),
        Transformation::ZRot => rotate_z(beacon),
        _ => {}
    }
}

pub fn get_matches(beacons : &Vec<Beacon>, map : &HashSet<Beacon>, trafos : &Mutation) -> usize {

    let mut shared = 0;
    for beacon in beacons {
        let mut b : Beacon = beacon.clone();
        b.apply(&trafos);

        if map.contains(&b) { shared += 1; }
    }

    shared 
}


// b - muster = x
// b - x = muster;
impl Translation {

    pub fn new(b1 : &Beacon, b2 : &Beacon) -> Self {
        Translation {
        x : b1.x - b2.x,
        y : b1.y - b2.y,
        z : b1.z - b2.z}
    }

    pub fn valid(&self) -> bool {
        self.x == self.y && self.y == self.z
    }

}
pub fn get_transformations(muster : &Beacon, current : &Beacon) -> Vec<Mutation> {
    let mut trafos = vec![];

    for trafo1 in Transformation::iter() {
        let mut b1 = (*current).clone();
        applyTrans(&mut b1, &trafo1);

        for trafo2 in Transformation::iter() {
            let mut b2 = b1.clone();
            applyTrans(&mut b2, &trafo2);

            for trafo3 in Transformation::iter() {
                let mut b3 = b2.clone();
                applyTrans(&mut b3, &trafo3);

                for trafo4 in Transformation::iter() {
                    let mut b4 = b3.clone();
                    applyTrans(&mut b4, &trafo4);

                    let t = Translation::new(&b4, muster);
                    trafos.push((trafo1.clone(), trafo2.clone(), trafo3.clone(), trafo4, t));
                }
            }
        }
    }

    trafos
}

impl Beacon
{
    pub fn new(line : &str) -> Self {
        let (x,y,z) : (isize, isize, isize) = line.splitn(3, ",").map(|x| x.parse().unwrap()).collect_tuple().unwrap();
        Beacon{ x : x, y : y, z : z}
    }

    pub fn sub(&mut self, t : &Translation) {
        self.x -= t.x;
        self.y -= t.y;
        self.z -= t.z;
    }
    pub fn apply(&mut self, trafos : &Mutation) {
        applyTrans(self, &trafos.0);
        applyTrans(self, &trafos.1);
        applyTrans(self, &trafos.2);
        applyTrans(self, &trafos.3);
        self.sub(&trafos.4);
    }


    pub fn manhatten(&self, other : &Beacon) -> usize{
       let xdiff = isize::abs(self.x - other.x);
       let ydiff = isize::abs(self.y - other.y);
       let zdiff = isize::abs(self.z - other.z);
       xdiff as usize + ydiff as usize + zdiff as usize
    }
}

#[derive(Debug)]
pub struct Scanner {
    inited : bool,
    pos : Beacon,
    beacons : Vec<Beacon>,
    map : HashSet<Beacon>,
}


impl Scanner {
    pub fn new(lines : &Vec<&str>, pos : &mut usize) -> Self {
        let mut beacons = vec![];

        while *pos < lines.len() {
            if lines[*pos].len() == 0 { break; }
            let beacon = Beacon::new(&lines[*pos]);
            println!("{}", beacon);
            beacons.push(beacon);
            *pos += 1;
        }
        println!();
        Scanner { inited : false, pos : Beacon{x : 0, y : 0, z : 0}, beacons : beacons, map : HashSet::new()}
    }

    pub fn init(&mut self) {
        self.inited = true;
        for i in 0..self.beacons.len() {
            self.map.insert(self.beacons[i].clone());
        }
    }

    pub fn apply(&mut self, t : Mutation) {
        self.pos.apply(&t);
       for i in 0..self.beacons.len() {
            self.beacons[i].apply(&t);
       }
       self.init();
    }

    pub fn manhatten(&self, other : &Scanner) -> usize {
        self.pos.manhatten(&other.pos)
    }
}

#[derive(Debug,PartialEq,Clone,Hash,Eq)]
pub struct Translation {
    x : isize,
    y : isize,
    z : isize,
}

impl fmt::Display for Translation{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "< {} {} {} >", self.x, self.y, self.z)
    }
}
fn main() {
    let files = vec!["data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let lines : Vec<&str>= input.lines().collect();

        let mut scanners = vec![];
        let mut pos = 1;
        while pos < lines.len() {
            let scanner = Scanner::new(&lines, &mut pos);
            scanners.push(scanner);
            pos += 2;
        }

        scanners[0].init();

        let mut todo : Vec<usize> = vec![0];

        while todo.len() != 0 {
            let refIdx = todo.remove(todo.len()-1);

            for scanIdx in 1..scanners.len() {
                if scanners[scanIdx].inited { continue; }

                let mut checked = HashSet::new();

                let lRef = scanners[refIdx].beacons.len();
                'search : for i in 0..lRef {

                    let lScan = scanners[scanIdx].beacons.len();
                    for j in 0..lScan {

                        let trafos = get_transformations(
                            &scanners[refIdx].beacons[i],
                            &scanners[scanIdx].beacons[j]);

                        for trafo in trafos {
                            if checked.contains(&trafo) { continue; }

                            let matches = get_matches(&scanners[scanIdx].beacons, &scanners[refIdx].map, &trafo);
                            if matches >= 12 {
                                println!("{} -> {}, {:?}", refIdx, scanIdx, trafo);
                                scanners[scanIdx].apply(trafo);
                                println!("scanner {}: {}", scanIdx, scanners[scanIdx].pos);
                                todo.push(scanIdx);
                                break 'search;
                            }

                            checked.insert(trafo);
                        }
                    }
                }
            }
            println!("");
        }
        
        let mut map = HashSet::new();
        let mut i = 0;
        for scanner in &scanners {
            if !scanner.inited { panic!("scanner {} not initalized!", i);}
             for b in &scanner.beacons {
                map.insert(b);
             }
             i+=1;
        }

        println!("star1: {}", map.len());

        let mut manhatten = 0;
        for i in 0..scanners.len() {
            for j in i+1..scanners.len() {
                manhatten = max(manhatten, scanners[i].manhatten(&scanners[j]));
            }
        }

        println!("star2: {}", manhatten);
        //for b1 in &scanner0.beacons {
        //    for b2 in &scanners[1].beacons {

        //        let trafo = is_transformation(&b1, &b2);
        //        if trafo != Transformation::No {
        //            println!("{:?} == {:?} with {:?}", b1,b2,trafo);
        //        }
        //    }

        //}

    }
}
