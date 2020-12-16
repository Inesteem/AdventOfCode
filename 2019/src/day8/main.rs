use lib::io::*;

fn main() {

    let w = 25;
    let h = 6;


    let mut line : Vec<char>= read_line_from_stdin().unwrap().trim().to_string().chars().collect();
    
    let num_layers = line.len() / (w * h);

    //let mut digits : Vec<Vec<Vec<u8>>> = vec![vec![vec![0;w];h];num_layers];
    let mut digits : Vec<Vec<u8>> = vec![vec![0;3];num_layers];

    assert!(num_layers * w * h == line.len());

    let mut i = 0;
    for layer in 0..num_layers {
        for row in 0..h {
            for col in 0..w {
                //digits[layer][row][col] = (line[i] as u8) - ('0' as u8);
                let d = (line[i] as u8) - ('0' as u8);
                i += 1;
                if d <= 2 {
                    digits[layer][d as usize] += 1; 
                }
            }
        }
    }
    digits.sort_by(|a, b| a[0].cmp(&b[0]));
    println!("{:?}", digits); 
    println!("star1: {}", (digits[0][1] as u16)  * (digits[0][2] as u16) );
}
