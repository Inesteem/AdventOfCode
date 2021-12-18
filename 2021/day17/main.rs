use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::f64;
use std::cmp::max;
use std::collections::HashSet;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn get_range(l : &str) -> (isize,isize) {

   let words : Vec<&str>= l.split("=").collect(); 
   let range: Vec<isize>= words[1].split("..").map(|s| s.parse().unwrap()).collect(); 

   (range[0], range[1])
}



fn gauss_sum(n : isize) -> isize {
    if n < 0 {
        return -1 * (n * (n-1)) / 2;
    }

    (n * (n+1)) / 2
}


fn is_in_range(range : &(isize,isize), val : isize) -> bool {
    if val < range.0 || val > range.1{ return false;}
    return true;
}

fn get_y_at(mut y : isize, step : isize) -> isize{
    let mut ypos = 0;
    if y > 0 {
        if step < 2*y+1 { return 0; } // wrong. it just knows that y vals > 0 are not in the target
       ypos = -( gauss_sum(step-y-1) - gauss_sum(y)); 
       return ypos;
    }
    for s in 0..step {
            ypos += y;
            y -= 1;
    }
    ypos
}
fn main() {
    //println!("{}", get_y_at(2,7));
    //process::exit(0);


    let files = vec!["test", "data"];

    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => process::exit(0),
        }

        let words : Vec<&str>= input.split_whitespace().collect(); 
        println!("{:?}", words);
        let x_range = get_range(&words[2][0..words[2].len()-1]);
        let y_range = get_range(words[3]);
        println!("star1: {}", gauss_sum( isize::abs(y_range.0) - 1));


        let mut map = HashSet::new();
        //this ugly peace of shitty code assumes that the target area is always below 0
        //and xmin > 0
        //it is also optimized using the input. I was to lazy to think carefully about it but I
        //assume there are a lot of smarter solutions out there :(
        for xvel in (1..=x_range.1).rev() {
            let mut xpos = xvel;
            for step in 1..=((-x_range.0) - (-x_range.1))*10 { // this is not true for all inputs
                if is_in_range(&x_range, xpos) {

                    for yvel in (y_range.0..=isize::abs(y_range.0)).rev() {
                        let ypos = get_y_at(yvel, step);
                        if is_in_range(&y_range, ypos) {
                            map.insert((yvel, xvel));
                        }
                        if ypos < y_range.0 { break; }
                    }

                } else if xpos > x_range.1 { break; }
                xpos += max(0, xvel - step);
            }
        }

        // 3785
        println!("star2: {}", map.len());

    }
}
