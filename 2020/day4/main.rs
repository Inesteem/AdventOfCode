use std::process;
extern crate lib;

use lib::io::read_str_from_file;
//
//struct Passport{

//    byr: u16,
//    iyr: u16,
//    eyr: u16,
//    hgt: str,
//    hcl: str,
//    ecl: str,
//    pid: usize,
//    cid: usize,
//}

//pub struct Field<'a>{
//    id: &'a str,
//    content: &'a str,
//}

#[derive(Debug)]
pub struct Field {
    pub id : String,
    pub content : String,
}

impl Field {
    
    pub fn new(info : & str) -> Field {
        println!(">{}<", info);
        let mut attrs : Vec<String> = info.split(':')
            .map(|x| x.to_string()).collect();
        let content = attrs.remove(1);
        let id = attrs.remove(0);
        Field {
            id: id,
            content : content,
        }
    }

}

//fn valid_passport(&mut info : str) -> bool{
//    let attrs : Vec<Field> = info.split(' ')
//        .map(|x| new Field(x.to_string())).collect();
//    true
//}


fn main() {
    let mut passports: Vec<Vec<Field>>;
    match read_str_from_file("data".to_string()) {
        Ok(i) =>  
            //let input_vec : Vec<u32> = inputs.split(",")
             passports = i.split('\n')
           // .map(|x| x.to_string().split(&[':',' '][..])
           //      .map(|y| y.to_string())
           //      .collect() 
           //      )
            .map(|x| x.split(' ')
                 .map(|x| match x {
                     "" => Field::new("-:-"), 
                     _ => Field::new(&x.to_string()),
                 }).take_while(|field| field.id.len() ==3)
                 .collect()
                 )
            //.take_while(|v| v.len() > 0)
            .collect()
            ,
        Err(_) => process::exit(0),
    };

    let mut vec :Vec<Vec<Field> >= Vec::new();
    passports.append(&mut vec);

    let mut num_valid : i32 = 0;
    let mut num  = 0;
    let mut cid = false;
    for i in 0..passports.len(){
        
        if passports[i].len() == 0 {
            print!("\n{} {} ",cid,num);
            if num == 8 || (num == 7 && !cid) {
                num_valid +=1;
                print!("-> valid");
            }
            println!("\n");
            num  = 0;
            cid = false;
        }
        
        for field in &passports[i] {
            num +=1;
            if field.id.eq("cid") {
                cid = true;
            }
            println!("field: {}", &field.id);
        }

    }
    println!("valid: {}", num_valid);
}
