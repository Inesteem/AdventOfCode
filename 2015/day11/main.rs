
fn cond1(password : &Vec<u8>) -> bool {
    for i in 2..password.len() {
        if (password[i-2] + 1 == password[i-1]) && 
            (password[i-1] + 1 == password[i]) { return true; }
    }
    false
}

const L : u8 = 'l' as u8 - 'a' as u8; 
const I : u8 = 'i' as u8 - 'a' as u8; 
const O : u8 = 'o' as u8 - 'a' as u8; 
fn cond2(password : &Vec<u8>) -> bool 
{
    for c in password {
        if *c == L || *c == I || *c == O { return false; }
    }

    true
}

fn cond3(password : &Vec<u8>) -> bool 
{
    let mut cnt = 0;
    let mut last = 123;
    for i in 1..password.len() {
        if password[i] == password[i-1] && password[i] != last {
            cnt += 1;
            last = password[i];
        }
    }

    cnt == 2
}

fn main() {
    let input = "hepxcrrq";
    let min_val = 'a' as u8;
    let max_val = 'z' as u8 - min_val;

    let mut password : Vec<u8> = input.chars().map(|c| (c as u8) - min_val).collect();

    for i in 0..2 {
        loop {
            for i in (0..password.len()).rev() {
                if password[i] == max_val {
                    password[i] = 0;
                    continue;
                }
                password[i] += 1;
                break;
            }

            if cond1(&password) && cond2(&password) && cond3(&password) { break; }

        }

        let output : String = password.iter().map(|c| ((min_val+c) as char).to_string()).collect::<Vec<String>>().join("");
        println!("star{}: {}", i+1, output);
    }
}
