fn main() {
    const number : usize = 34000000 / 10;

    //let mut numbers : Vec<usize> = vec![0;10];
    let mut numbers : Vec<usize> = vec![0;10000000];
    let houses = numbers.len();
    for i in 0..houses {
        for n in (i..houses).step_by(i+1) {
            numbers[n] += i + 1;
        }
    }
//    println!("{:?}", numbers);
    for i in 0..houses {
        if numbers[i] >= number {
            println!("star1 {}", i+1);
            break;
        }
    }

}
