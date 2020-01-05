fn main() {
    println!("Hello, world!");

    let lower_bound = 197487;
    let upper_bound = 673251;
    let mut num = lower_bound;
    let mut pws = 0;
    let pairs  = vec!["22","33","44","55","66","77","88","99"];
    let triples= vec!["222","333","444","555","666","777","888","999"];
    loop {
     
        let mut digits : Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        for i in 1..6 {
            while digits[i] < digits[i-1] {
                digits[i]+=1;
            }
        }
        let s: String = digits.into_iter().map(|d| d.to_string()).collect();
        num = s.parse::<i32>().unwrap();

        if num > upper_bound {
            break;
        }

        for i in 0..8 {
            if s.contains(pairs[i]) && !s.contains(triples[i]) {
                pws+=1;
                break; 
            }
        }

        num+=1;
    }
    println!("num pws {}" , pws);
}
