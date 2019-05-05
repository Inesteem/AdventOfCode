
fn checksum(data: &mut Vec<char>){

    while data.len() % 2 == 0 {
        let mut v = Vec::new();

        for i in (0..data.len()-1).step_by(2){

            if data[i] == data[i+1] {
                v.push('1');
            } else {
                v.push('0');
            }
        }
        *data = v;
    
    }

}




fn main() {

    let disc_len = 35651584;
    let state = "10010000000110000";

    let s: Vec<char> = state.chars().collect();
    let mut data = s.clone();

    while data.len() < disc_len {
        data.push('0');
        for i in (0..data.len()-1).rev(){
            if data[i] == '0'{
                data.push('1');
            } else {
                data.push('0');
            }
        }
    
    }
    data.resize(disc_len,'x');

    let res: String = data.iter().collect();
    println!("data: {}", res);

    checksum(&mut data);

    let crc: String = data.iter().collect();
    println!("crc: {} with len {}", crc, crc.len());


}
