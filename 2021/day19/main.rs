use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

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

pub fn get_matches(beacons : &Vec<Beacon>, map : &HashMap<Beacon, usize>, trafos : &(Transformation,Transformation, Transformation, isize)) -> usize {

    let mut num = 0;
    for beacon in beacons {
        let mut b : Beacon = beacon.clone();
        applyTrans(&mut b, &trafos.0);
        applyTrans(&mut b, &trafos.1);
        applyTrans(&mut b, &trafos.2);
        if map.contains_key(&b) { num += 1; }
    }

    num
}



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

pub fn basic_check(b1 : &Beacon, b2 : &Beacon) -> bool {
   let mut v1 = vec![isize::abs(b1.x),isize::abs(b1.y),isize::abs(b1.z)];
   let mut v2 = vec![isize::abs(b2.x),isize::abs(b2.y),isize::abs(b2.z)];
   v1.sort();
   v2.sort();
   if v1[1] - v1[0] != v2[1] - v2[0] {return false;}
   if v1[2] - v1[0] != v2[2] - v2[0] {return false;}
   true
}

pub fn get_transformations(muster : &Beacon, current : &Beacon) -> Vec<(Transformation,Transformation,Transformation, isize)> {
    let mut trafos = vec![];
    if !basic_check(muster,current) { return trafos; }
    for trafo1 in Transformation::iter() {
        let mut b1 = (*current).clone();
        applyTrans(&mut b1, &trafo1);

        for trafo2 in Transformation::iter() {
            let mut b2 = b1.clone();
            applyTrans(&mut b2, &trafo2);

            for trafo3 in Transformation::iter() {
                let mut b3 = b2.clone();
                applyTrans(&mut b3, &trafo3);

                let t = Translation::new(&b3, muster);
                if t.valid() {
                    trafos.push((trafo1.clone(), trafo2.clone(), trafo3, t.x));
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

}

#[derive(Debug)]
pub struct Scanner {
    beacons : Vec<Beacon>,
    map : HashMap< Beacon, usize>,
}


impl Scanner {
    pub fn new(lines : &Vec<&str>, pos : &mut usize) -> Self {
        let mut beacons = vec![];
        let mut map = HashMap::new();
        let mut rng = rand::thread_rng();
        let n1 : isize = rng.gen::<i8>() as isize;

        while *pos < lines.len() {
            if lines[*pos].len() == 0 { break; }
            let mut beacon = Beacon::new(&lines[*pos]);

            map.insert(beacon.clone(), beacons.len());
            beacons.push(beacon);
            *pos += 1;
        }
        println!();
        Scanner { beacons : beacons, map : map }
    }
}

#[derive(Debug,PartialEq,Clone,Hash)]
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
    let files = vec!["simple"];
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

        let scanner0 = &scanners[0];

        for scanner in &scanners[1..scanners.len()] {
            let mut checked = HashSet::new();
            for i in 0..scanner0.beacons.len() {
                for j in 0..scanner.beacons.len() {
                    let b1 = &scanner0.beacons[i];
                    let b2 = &scanner.beacons[j];
                    let trafos = get_transformations(b1, b2);

                    for trafo in trafos {
                        if checked.contains(&trafo) { continue; }

                        let matches = get_matches(&scanner.beacons, &scanner0.map, &trafo);
                        println!("{:?}: {}", trafo, matches);

                        checked.insert(trafo);
                    }
                }
            }
            println!("");
        }

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
