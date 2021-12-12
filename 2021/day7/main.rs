use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::cmp::min;


fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn star1(numbers : Vec<usize>) {
    let len = numbers.len();
     let mut costs = vec![0; numbers[len-1]+1];
    
    let mut numPrev = 0;
    let mut pos = 1;
    for n in &numbers {
        for p in pos..=*n {
            costs[p] += costs[p-1] + numPrev;
        }
        pos = *n+1;
        numPrev += 1;
    }

    numPrev = 0;
    pos = len-1;
    let mut prevCosts = 0;
    let mut minCosts = costs.last().unwrap() * 2;
     for n in (0..len).rev() {
        for p in (numbers[n]..=pos).rev() {
            prevCosts += numPrev; 
            costs[p] += prevCosts;
            minCosts = min(minCosts, costs[p]);
        }
        if numbers[n] == 0 {
            break;
        }
        pos = numbers[n]-1;
        numPrev += 1;
    }
    println!("{:?}", costs);
    println!("-> {}", minCosts);


}

fn main() {

    let mut numbers: Vec<usize>;
    match read_inputs("data".to_string()) {
        Ok(inputs) =>  
            numbers = inputs.split(",")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect(),
        Err(_) => process::exit(0),
    }

    numbers.sort();
    println!("{:?}", numbers);
    let len = numbers.len();
    
    let mut costsForward = vec![0; numbers[len-1]+1];
    
    let mut pos : i32 = 0;
    while pos < (len as i32) {
        let num = numbers[pos as usize];
        pos += 1;
        let mut numPrev = 1;

        while pos < (len as i32) && numbers[pos as usize] == num {
            numPrev += 1;
            pos += 1;
        }
        
        let mut costsPrev = 0;
        
        for p in (num+1)..=numbers[len-1] {
            costsPrev += numPrev * (p - num);
            costsForward[p] += costsPrev;
        }

    }

    println!("{:?}", costsForward);

 
    let mut costsBackward = vec![0; numbers[len-1]+1];
    
    pos = (len as i32) - 1;
    while pos >= 0 {
        let num = numbers[pos as usize];
        pos -= 1;
        let mut numPrev = 1;

        while pos >= 0 && numbers[pos as usize] == num {
            numPrev += 1;
            pos -= 1;
        }
        
        let mut costsPrev = 0;
        for p in (0..num).rev() {
            costsPrev += numPrev * (num - p);
            costsBackward[p] += costsPrev;
        }
    }

    println!("{:?}", costsBackward);
 
    let mut minCost = costsBackward[0]*2;
    for i in 0..=numbers[len-1]
    {
        minCost = min(minCost, costsBackward[i] + costsForward[i]);
    }
    
    println!("{}", minCost);
   }

//[0, 1, 1, 2, 2, 2, 4, 7, 14, 16]
//0 1 2 3  4  5  6  7  8  9  A B C D E F 10
//1 2 3 0  1  0  0  1  0  0  0 0 0 0 1 0 1
//0 1 3 6  6  7  7  7  8  8  8 8 8 8 8 9 9  10
//0 1 4 10 16 23 30 37 45 53 
