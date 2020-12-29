

fn main() {
//    let mut ring = vec![3,8,9,1,2,5,4,6,7];
    let mut ring = vec![4,6,9,2,1,7,5,3,8];
    let highest = *ring.iter().max().unwrap();
    let lowest = *ring.iter().min().unwrap();
    let mut cur_idx = 0;


    for _i in 0..100 {
        print!("-- move {} --\ncups: ", _i+1);
        for j in 0..ring.len() {
            if j == cur_idx {
                print!("({}) ", ring[j]);
            } else {
                print!("{} ", ring[j]);
            }
        }
        println!("\npickup: ");

        let mut v = vec![];
        for j in 0..3 {
            let rem_idx = (cur_idx+1)%ring.len();
            if rem_idx < cur_idx { cur_idx -=1;}
            //print!("- {} ", (cur_idx+1)%ring.len());
            let elem = ring.remove(rem_idx);
            v.push(elem);
            print!("{} ", elem);
        }
        println!("");
        let mut dest_idx = -1;
        let mut comp = ring[cur_idx]; 
        while dest_idx < 0  {
            if comp + dest_idx < lowest {
                comp = highest+1;
                dest_idx = -1;
            }
            for j in 0..ring.len() {
                if ring[j] == comp + dest_idx {
                    dest_idx = j as i32;
                    break;
                }
            }
            if dest_idx < 0 { dest_idx -=1; }
        }
        println!("dest: {}", ring[dest_idx as usize]);
        for j in 0..v.len() {
            ring.insert((dest_idx as usize +j+1),v[j]);
        }
        if dest_idx < cur_idx as i32 { cur_idx += 3; }
        
        //select next current cup
        cur_idx = (cur_idx +1) % ring.len();
        println!("");
    }

    let one_idx = ring.iter().position(|&r| r == 1).unwrap();;
    print!("\nstar1: ");
    for i in 1..ring.len() {
        print!("{}", ring[(i+one_idx) % ring.len()]);
    }
    println!("");
}
