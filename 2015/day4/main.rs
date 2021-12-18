use std::process;
use crypto::md5::Md5;
use crypto::digest::Digest;
fn is_zero_star1(output : &[u8]) -> bool {
    if output[0] != 0 || output[1] != 0{ return false; }
    if output[2] & 0xF0 != 0 { return false; }
    true
}
fn is_zero_star2(output : &[u8]) -> bool {
    if output[0] != 0 || output[1] != 0{ return false; }
    if output[2] != 0 { return false; }
    true
}

fn main() {
    let keys = vec!["abcdef", "pqrstuv", "bgvyzdsv"];
    for key in keys {
        println!("{}", key);

        let mut magic = Md5::new();
        let data = key.as_bytes();

        let mut n = 0;
        loop {
            magic.input(data);
            magic.input(n.to_string().as_bytes());
            let mut output = [0; 16]; 
            magic.result(&mut output);
            if is_zero_star2(&output) {
                println!("{}", n);
                break;
            }
            magic.reset();
            n += 1;
        }
    }
}
