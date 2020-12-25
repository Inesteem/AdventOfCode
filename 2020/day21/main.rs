use lib::io::*;
use std::collections::HashMap;

fn main() {
    let mut lines : Vec<String> = vec![];
    let mut map = HashMap::new();
    
    let mut all_ingr = HashMap::new();

    loop {
        lines.push(read_line_from_stdin().unwrap());
        if lines[lines.len()-1].len() == 0 { break; }
        print!("{}", lines[lines.len()-1]);
    }

    //map ingredient vec to allergene
    //remove ingredients that aren't always mentioned for an allergene
    for i in 0..lines.len() {
        //has allergenes?
        let s : Vec<&str> = lines[i].split(" (contains ").collect();
        if s.len() == 1 { continue; }
        let ingredients : Vec<&str> = s[0].split_whitespace().collect();
        
        //add ingredients to global all_ingredients map
        for ingr in &ingredients {
            if !all_ingr.contains_key(*ingr){
                all_ingr.insert(ingr.to_string(),1);
            } else {
                *(all_ingr.get_mut(*ingr).unwrap()) += 1;
        
            }
        }

        let allergenes : Vec<&str> = s[1][0..s[1].len()-2].split(", ").collect();
        println!("{:?} -> {:?}", ingredients, allergenes);
        for a in &allergenes {
            if !map.contains_key(&a[..]) {
                map.insert(a.to_string(), ingredients.clone()); 
            } else {
                let mut j = 0;
                loop {
                    let ingr = map.get_mut(&a[..]).unwrap();
                    let len = ingr.len();
                    if j >= len { break; } 
                    if ingredients.contains(&ingr[j]) {
                        j += 1; 
                    } else {
                        println!("ingredient {} not important for allergene {}", &ingr[j], &a);  
                        ingr.remove(j);
                    }

                }
            }
        }
    }  

    //1 to 1 mapping; remove ingredients that are obviously mapped to one allergene from the lists
    //  of other allergenes
    let mut to_remove : Vec<&str>= vec![]; 
    loop {
        let mut finished = true;
        let mut progress = 0;
        for ingr in map.values_mut() {
            let l = ingr.len();
            assert!(l > 0);
            if l > 1 {
                finished = false;
                for item in &to_remove {
                    ingr.retain(|e| !e.eq(item));
                }
                if ingr.len() != l { progress += 1; }
            }
            if ingr.len() == 1 && !to_remove.contains(&ingr[0]){
                to_remove.push(ingr[0]);    
                progress += 1;
            }
        }
        println!(""); 
        if finished { break; }
        if progress == 0 {
          println!("no more progress");
          break;
        }
    }    

    //
    println!("{:?}", map);
    println!("{:?}", &all_ingr);

    let allergic_ingredients : Vec<&str>= map.values().map(|x| x[0]).collect();
    println!("{:?}", allergic_ingredients);
    let mut star1 = 0;
    for i in all_ingr.keys(){
        if !allergic_ingredients.contains(&&i[..]) {
            star1 += all_ingr.get(i).unwrap();
        }
    }
    println!("star1: {}", star1);
}
