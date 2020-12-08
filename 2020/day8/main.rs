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

    let mut i : usize = 0;
    let mut acc : i32 = 0;
    let mut seen = vec![false; operations.len()];
    loop {
        let (comm, num) = &operations[i];
        println!("{} {}", &comm, &num);
        if seen[i] { break; }
        seen[i] = true;
        match comm.as_str() {
            "nop" => i+=1usize,
            "jmp" => i = (i as i32 + *num) as usize,
            "acc" => {acc+=num; i+=1usize;},
            _ => (),
        }
    }
    println!("{}", acc);

}
