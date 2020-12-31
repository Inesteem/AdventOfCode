
fn transform(s_num : usize, mut v : usize) -> usize {
    v *= s_num;
    v %= 20201227;
    v
}

fn get_loop_num(s_num : usize, card_pub : usize) -> usize {
    let mut loop_num = 1;
    let mut v = s_num;
    while v !=  card_pub {
        loop_num += 1;
        v=transform(s_num, v);
    }
    loop_num 
}

fn main() {
    let s_num = 7;
//    let card_pub= 5764801;
//    let door_pub= 17807724;

    let card_pub = 10604480;
    let door_pub = 4126658;

    let lc = get_loop_num(s_num, card_pub);
    let ld = get_loop_num(s_num, door_pub);

    let mut encr_key = door_pub;
    println!("loop {}", lc);

    for i in 0..lc-1 {
        encr_key = transform(door_pub, encr_key);
    }
        println!("encr_key : {}", encr_key);
    

}
