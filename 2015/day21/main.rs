use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::isize;
use std::cmp::{min,max};
use std::collections::{HashSet, HashMap};

const noItem : Item = Item{ cost : 0, damage : 0, armor : 0};

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

#[derive(Debug,Clone)]
struct Item {
    cost : isize,
    damage : isize,
    armor : isize,
}

#[derive(Clone, Debug)]
struct Player {
    health : isize,
    damage : isize,
    armor : isize,
}

impl Player {
    fn getHitBy(&mut self, enemy : &Player) {
        self.health -= max(1, enemy.damage - self.armor);
    }

    fn alive(&self) -> bool {
        self.health > 0
    }

}

fn playerWins(player : &mut Player, boss : &mut Player) -> bool {
    loop {
        boss.getHitBy(&player);
        if !boss.alive() {
            break;
        }
        player.getHitBy(&boss);
        if !player.alive() {
            break;
        }
    }
    player.alive()
}


fn bestDuell(star : u8,
             mut player : Player,
             mut boss : Player,
             pos : usize,
             inventory : &mut Vec<Selection>,
             money : &mut isize) {

    if pos == inventory.len() {
        let mut currMoney = 0;
         for i in 0..inventory.len()
        {
            inventory[i].apply(&mut player);
            currMoney += inventory[i].getCost();
        }

        if star == 1 
        {
            if playerWins(&mut player, &mut boss) 
            {
                *money = min(*money, currMoney);
            }
        } else {
            if !playerWins(&mut player, &mut boss) 
            {
                *money = max(*money, currMoney);
            }
        }
        return;
    }

    let selection = inventory[pos].selected;
    while !inventory[pos].finished() {
        bestDuell(star, player.clone(), boss.clone(), pos + 1, inventory, money);
        inventory[pos].incr();
    }
    inventory[pos].selected = selection;
}

#[derive(Debug, Clone)]
struct Selection {
    items : Vec<Item>,
    selected : isize,
    max : isize,
}

impl Selection {

    fn incr(&mut self) {
        self.selected += 1;
    }

    fn decr(&mut self) {
        self.selected -= 1;
    }

    fn canBuy(&self, money : isize) -> bool {
        self.getCost() <= money
    }

    fn getCost(&self) -> isize {
        if self.selected < 0 || self.selected as usize >= self.items.len() { return 0; }
        self.items[self.selected as usize].cost

    }

    fn apply(&self, player : &mut Player) {
        if self.selected < 0 || self.selected as usize >= self.items.len() { return; }

        player.damage += self.items[self.selected as usize].damage;
        player.armor += self.items[self.selected as usize].armor;
    }

    fn deapply(&self, player : &mut Player) {
        if self.selected < 0 || self.selected as usize >= self.items.len() { return; }

        player.damage -= self.items[self.selected as usize].damage;
        player.armor -= self.items[self.selected as usize].armor;
    }

    fn finished(&self) -> bool { return self.selected == min(self.max, self.items.len() as isize -1); }
}
fn main() {

    let mut weapons = Selection{ items : vec![
        Item{cost : 8, damage : 4, armor : 0},
        Item{cost :10, damage : 5, armor : 0},
        Item{cost :25, damage : 6, armor : 0},
        Item{cost :40, damage : 7, armor : 0},
        Item{cost :74, damage : 8, armor : 0},
    ], selected : 0 , max : 4};
    let mut armor = Selection{ items : vec![
        Item{cost :  13, damage : 0, armor : 1},
        Item{cost :  31, damage : 0, armor : 2},
        Item{cost :  53, damage : 0, armor : 3},
        Item{cost :  75, damage : 0, armor : 4},
        Item{cost : 102, damage : 0, armor : 5},
    ], selected : -1 , max : 5 };
    let mut rings = Selection{ items : vec![
        Item{cost :  25, damage : 1, armor : 0 },
        Item{cost :  50, damage : 2, armor : 0 },
        Item{cost : 100, damage : 3, armor : 0 },
        Item{cost :  20, damage : 0, armor : 1 },
        Item{cost :  40, damage : 0, armor : 2 },
        Item{cost :  80, damage : 0, armor : 3 },
    ], selected : -1  , max : 6};

    let mut selections = vec![weapons, armor, rings.clone(), rings];

    let files = vec!["data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }
        let lines : Vec<isize> = input.lines().map(|x| x.split(": ").collect::<Vec<&str>>()[1].parse().unwrap()).collect();
        let mut boss = Player {health : lines[0], damage : lines[1], armor : lines[2]};

        let mut player = Player {health : 100, damage : 0, armor : 0};
        {
            let mut money = 1000;
            bestDuell(1,player.clone(), boss.clone(), 0, &mut selections, &mut money);
            println!("star1: {}", money);
        }
        let mut money = 0;
        bestDuell(2,player, boss, 0, &mut selections, &mut money);
        println!("star2: {}", money);
    }




}
