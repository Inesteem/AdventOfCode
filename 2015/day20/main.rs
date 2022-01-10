fn main() {
    const number : usize = 34000000;

    let mut numbers : Vec<usize> = vec![0;10000000];
    let houses = numbers.len();
    for i in 0..houses {
        for n in (i..houses).step_by(i+1) {
            numbers[n] += (i + 1) * 10;
        }
    }
    for i in 0..houses {
        if numbers[i] >= number {
            println!("star1 {}", i+1);
            break;
        }
    }
    
    numbers = vec![0;10000000];
    for i in 0..houses {
        let mut num = 0;
        for n in (i..houses).step_by(i+1) {
            numbers[n] += (i + 1) * 11;
            num += 1;
            if num == 50 { break; }
        }
    }
    for i in 0..houses {
        if numbers[i] >= number {
            println!("star2 {}", i+1);
            break;
        }
    }

}
