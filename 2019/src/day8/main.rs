use lib::io::*;

fn main() {

    let w = 25;
    let h = 6;


    let mut line : Vec<char>= read_line_from_stdin().unwrap().trim().to_string().chars().collect();
    
    let num_layers = line.len() / (w * h);

    let mut image: Vec<Vec<u8>> = vec![vec![2;w];h];
    let mut digits : Vec<Vec<u8>> = vec![vec![0;3];num_layers];

    assert!(num_layers * w * h == line.len());

    let mut i = 0;
    for layer in 0..num_layers {
        for row in 0..h {
            for col in 0..w {
                //digits[layer][row][col] = (line[i] as u8) - ('0' as u8);
                let d = (line[i] as u8) - ('0' as u8);
                //star1
                if d <= 2 {
                    digits[layer][d as usize] += 1; 
                }
                //star2
                if image[row][col] == 2 {
                    image[row][col] = d;
                }
                i += 1;
            }
        }
    }
    digits.sort_by(|a, b| a[0].cmp(&b[0]));
    println!("{:?}", digits); 
    println!("star1: {}", (digits[0][1] as u16)  * (digits[0][2] as u16) );
    
    println!("");
    for row in &image {
        for col in row {
            if *col == 1 {
                print!("{}", '#');
            } else {
                print!("{}", '-');
            }
        } 
        println!("");
    }

}
