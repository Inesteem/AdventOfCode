use std::process;
extern crate lib;

use lib::io::read_str_from_file;
use regex::Regex;


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
fn in_range(content: & str, min : i32, max : i32) -> bool{ 
    match content.parse::<i32>() {
        Ok(ref v) => *v >= min && *v <= max,
        Err(_) => false,
    }
}

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
    let mut invalid = false;
    for i in 0..passports.len(){
        
        if passports[i].len() == 0 {
            //print!("\n{} {} ",cid,num);
            if !invalid && (num == 8 || (num == 7 && !cid)) {
                num_valid +=1;
                //print!("-> valid");
            }
            //println!("\n");
            num  = 0;
            cid = false;
            invalid = false;
        }
        if invalid {continue;}
        for field in &passports[i] {
            num +=1;
            match &field.id[..] {//todo:if invalid:break
                "cid" => cid = true,
                "byr" => {
                    if !in_range(&field.content[..],1920, 2002) {
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    }
                },
                "iyr" => {
                    if !in_range(&field.content[..], 2010, 2020) {
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    }
                },
                "eyr" => {

                    if !in_range(&field.content[..], 2020, 2030) {
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    }
                },
                "hgt" => {
                    let l = field.content.len();
                    if l < 4 { 
                        invalid = true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    }
                    let ext = &field.content[l-2..l];
                    if ext.eq("cm"){
                        if !in_range(&field.content[0..l-2],150,193){
                            invalid=true;
                            println!("invalid-> {} : {}", &field.id, &field.content);
                            break;
                        }
                    } else if ext.eq("in"){
                        if !in_range(&field.content[0..l-2],59,76){
                            invalid=true;
                            println!("invalid-> {} : {}", &field.id, &field.content);
                            break;
                        }
                     } else { 
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    }
                },
                "hcl" => {
                    let re = Regex::new("^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
                    if !re.is_match(&field.content){ 
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    } 
                },
                "ecl" => {
                    if !["amb","blu","brn","gry","grn","hzl","oth"].contains(&&field.content[..]) {
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    }
                },
                "pid" => {

                    let re = Regex::new(r"^\d{9}$").unwrap();
                    if !re.is_match(&field.content){ 
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    } 
                },
                _ => { 
                        invalid=true;
                        println!("invalid-> {} : {}", &field.id, &field.content);
                        break;
                    },
            }
            //println!("valid {}", &field.content);
            //println!("field: {}", &field.id);
        }

    }
    println!("valid: {}", num_valid);
}
