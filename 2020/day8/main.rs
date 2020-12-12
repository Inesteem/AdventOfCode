use std::io;
use std::option::Option;
use input_stream::InputStream;

fn read_comm() -> Option<(String, i32)> {
    let stdin = io::stdin();
    let mut input = InputStream::new(stdin.lock());

    let comm  = input.scan::<String>();
    let num = input.scan::<i32>();
    match  num{
        Ok(_) => (), 
        Err(_) => return None,
    }

    Some((comm.expect("command"), num.expect("num")))
}

fn main() {
     let mut operations = Vec::<(String,i32)>::new();
     loop {
        match read_comm() {
            None => break,
            //Some((comm,num)) => println!("{} {}", comm, num),
            Some(v) => operations.push(v),
        }
    }

    let (_,acc) = check_end(&mut operations);
    println!("Star1: {}", acc);

    for i in 0..operations.len() {
        let (comm_b, num_b) = &operations[i];
        let comm = comm_b.to_string();
        let num = *num_b;

        if comm.eq("nop") {
            operations[i] = ("jmp".to_string(),num);
        } else if comm.eq("jmp"){
            operations[i] = ("nop".to_string(),num);
        }

        let (fin,acc) = check_end(&mut operations);
        if fin {
            println!("Star2: {}",acc);
            break;
        }
        
        operations[i] = (comm.to_string(),num);
    }

}

fn check_end(operations : &mut Vec<(String,i32)>) -> (bool, i32){
    let mut i : usize = 0;
    let mut acc : i32 = 0;
    let mut seen = vec![false; operations.len()];
    loop {
        if i >= operations.len()  { return (true,acc); }
        let (comm, num) = &operations[i];
        //println!("{} {}", &comm, &num);
        if seen[i] { break; }
        seen[i] = true;
        match comm.as_str() {
            "nop" => i+=1usize,
            "jmp" => i = (i as i32 + *num) as usize,
            "acc" => {acc+=num; i+=1usize;},
            _ => (),
        }
    }
    (false, acc)
}
