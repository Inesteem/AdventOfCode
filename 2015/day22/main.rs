use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;
use std::{isize,usize};
use std::cmp::{max, min};

//TODO: BUGGY - IT DOES NOT WORK!
//need to fix this shit

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn read_in_pos() -> usize {
    get_input().trim().parse::<usize>().unwrap()
}

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

#[derive(Clone, Debug, PartialEq)]
enum SpellType {
    INSTANT,
    STEADY,
    ONE_TIME,
}

#[derive(Debug,Clone)]
struct Spell {
    active : isize,
    turns : usize,
    cost : isize,
    damage : isize,
    armor : isize,
    health : isize,
    mana : isize,
    s_type : SpellType,
}
impl Spell {
    fn inactivate(&mut self){
        self.active = 0;
    }

    fn activate(&mut self){
        self.active = self.turns as isize + 1;
    }

    fn is_effect(&self) -> bool {
        return self.s_type != SpellType::INSTANT;
    }
    fn is_instant(&self) -> bool {
        return self.s_type == SpellType::INSTANT;
    }
    fn is_steady(&self) -> bool {
        return self.s_type == SpellType::STEADY;
    }

    fn active(&self) -> bool {
        return self.active > 0
    }

    fn next_turn(&mut self) {
        self.active -= 1;
    }

    fn apply_bonus(&self, player : &mut Player, dir : isize) {
        player.armor += self.armor * dir;
        player.health += self.health * dir;
        player.mana += self.mana * dir;
    }

    fn remove(&self, player : &mut Player) {
        self.apply_bonus(player, -1);
    }

    fn apply(&self, player : &mut Player, enemy : &mut Player) {
        self.apply_bonus(player, 1);
        enemy.health -= self.damage;
    }

    fn print(&self, name : &str) {
        println!("{}: cost: {}, damage: {}, armor: {}, health: {}, mana: {}, timer: {}", name, self.cost, self.damage, self.armor, self.health, self.mana, self.active);
    }
}
#[derive(Clone, Debug)]
struct Player {
    health : isize,
    damage : isize,
    armor : isize,
    mana : isize,
    min_damage : isize,
}

impl Player {
    fn getHitBy(&mut self, enemy : &Player) {
        self.health -= max(enemy.min_damage, enemy.damage - self.armor);
    }

    fn alive(&self) -> bool {
        self.health > 0
    }

    fn buy(&mut self, spell: & Spell) {
        self.mana -= spell.cost;
    }
    fn can_buy(&self, spell : &Spell) -> bool {
        spell.cost <= self.mana
    }
    fn print(&self, name : &str) {
        println!("{}: hitpoints: {}, armor: {}, mana: {}", name, self.health, self.armor, self.mana);
    }
}

fn manage_effects(spells : &mut Vec<Spell>, player : &mut Player, boss : &mut Player)
{
    player.print("player");
    boss.print(  "boss  ");
    for i in 0..spells.len() {
        if spells[i].active() && spells[i].is_steady(){
            spells[i].apply(player, boss);
            spells[i].print("apply");
        }
        if spells[i].active == 1 {
            if spells[i].is_effect() {
                spells[i].remove(player);
                print_name("remove", i);
            }
        }
        spells[i].next_turn();
    }
    println!();
}
//too high -> 992
fn battle(mut player : Player, mut boss : Player, spells : &mut Vec<Spell>, mana : usize, minMana : &mut usize) {

    manage_effects(spells, &mut player, &mut boss);
    if !boss.alive() { *minMana = min(*minMana, mana); return; }

    player.getHitBy(&boss);
    if !player.alive() { return; }

    manage_effects(spells, &mut player, &mut boss);
    if !boss.alive() { *minMana = min(*minMana, mana); return; }

    return epic_battle(player, boss, spells, mana, minMana);
}

fn print_name(msg: &str, i : usize) {
    let names= vec!["magic missile", "drain", "shield", "poison", "recharge", "no"];
    println!("{}: {}",msg, names[i]);
}
fn epic_battle(player : Player, boss : Player, spells : &mut Vec<Spell>, mana : usize, minMana : &mut usize){
    if mana >= *minMana { return; }
    for i in 0..spells.len() {
//        let  i = read_in_pos();
        if spells[i].active() || !player.can_buy(&spells[i]) { continue; }

        let mut nextPlayer = player.clone();
        let mut nextBoss = boss.clone();

        nextPlayer.buy(&spells[i]);
//        spells[i].print("buy");
        print_name("buys ", i);
        if !spells[i].is_instant() {
            spells[i].activate();
        }
        if !spells[i].is_steady() {
            spells[i].print("apply");
            spells[i].apply(&mut nextPlayer, &mut nextBoss);
        }

        
        battle(nextPlayer, nextBoss, spells, mana+spells[i].cost as usize, minMana);

        spells[i].inactivate(); 
//        break;
    }

}


fn main() {

    let mut spells = vec! [
         Spell{s_type : SpellType::INSTANT, active : 0, turns : 1, 
             cost : 53,  damage : 4, armor : 0, health : 0, mana : 0}, //0 Magic Missile

         Spell{s_type : SpellType::INSTANT, active : 0, turns : 1, 
             cost : 73,  damage : 2, armor : 0, health : 2, mana : 0}, //1 Drain

         Spell{s_type : SpellType::ONE_TIME, active : 0, turns : 6,
             cost : 113, damage : 0, armor : 7, health : 0, mana : 0}, //2 Shield

         Spell{s_type : SpellType::STEADY, active : 0, turns : 6, 
             cost : 173, damage : 3, armor : 0, health : 0, mana : 0}, //3 Poison

         Spell{s_type : SpellType::STEADY, active : 0, turns : 5, 
             cost : 229, damage : 0, armor : 0, health : 0, mana : 101}, //4 Recharge

//         Spell{s_type : SpellType::INSTANT, active : 0, turns : 1, 
//             cost : 0,  damage : 0, armor : 0, health : 0, mana : 0}, //5 5 noo
    ];
    let file="data";
    let input: String;
    match read_inputs(file.to_string()) {
        Ok(inputs) =>
            input = inputs,
        Err(_) => process::exit(-1),
    }
    let lines : Vec<isize> = input.lines().map(|x| x.split(": ").collect::<Vec<&str>>()[1].parse().unwrap()).collect();
    //TODO : set stats
    let mut boss = Player {health : lines[0], damage : lines[1], armor : 0, mana : 250, min_damage : 1};

    let mut player = Player {health : 50, damage : 0, armor : 0, mana : 500, min_damage : 0};

    //boss.health = 14;
    //boss.damage = 8;
    //player.health = 10;
    //player.mana = 250;

    let mut manaUsed = usize::MAX;
    epic_battle(player, boss, &mut spells, 0, &mut manaUsed);
    println!("{}", &mut manaUsed);

}
