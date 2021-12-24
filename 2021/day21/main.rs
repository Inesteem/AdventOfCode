use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use itertools::Itertools;

fn read_inputs(filename : String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn star1(pos1 : u64, pos2 : u64) {
    //mit jedem Zug 3 * 6 = 18 mehr 7
    //6 24 42 60 78 96 | 14 32 50 68 86 | 4 22 40 58 76 94 | 12 30 48 66 84 | 2
    //6 2  9  5  1  8  | 3  10 6  2  9    4 0  7  3  10 6
    let mut i = 0;
    let mut num1 = pos1-1;
    let mut num2 = pos2-1;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut playered = 0;
    loop {
        for j in 0..3 {
            print!("{} ", i+1);
            num1 += i+1;
            i = (i+1) % 100;
        }
        num1 %= 10;
        score1 += num1+1;
        println!(" -> {} {}", num1+1, score1);

        playered += 3;
        if score1 >= 1000 {
            println!("{} {} {}", playered, score2, score2 * playered);
            return;
        }
        for j in 0..3 {
            print!("{} ", i);
            num2 += i+1;
            i = (i+1) % 100;
        } 
        num2 %= 10;
        score2 += num2+1;
        println!(" -> {} {}\n", num2, score2);

        playered += 3;
        if score2 >= 1000 {
            println!("{} {} {}", playered, score1, score1 * playered);
            return;
        }
    }
}

#[derive(Debug,Clone,Copy)]
struct Player {
    pos : usize,
    score : usize,
}

impl Player {
    fn incr(&self, num : usize) -> Self{
        let pos = (self.pos + num) % 10;
        let score = self.score + pos + 1;
        Player{ pos : pos, score : score }
    }
}

fn get_idx(player: usize, players : &[Player;2]) -> usize {
    let mut idx = player * (10 * 10 * 21 * 21);
    idx += players[0].pos * (10 * 21 * 21);
    idx += players[0].score * (10 * 21);
    idx += players[1].pos * 21;
    idx += players[1].score;
    idx
}

fn diracDice(player: usize, mut players: [Player;2], dp : &mut Vec<(i64,i64)>) -> (i64,i64) {
    if players[0].score >= 21 { return (1,0); }
    if players[1].score >= 21 { return (0,1); }

    let idx = get_idx(player, &players);
    if dp[idx].0 < 0 {

        let nextPlayer = player^1;
        let p = players[player].clone();

        for i1 in 1..=3 {
            for i2 in 1..=3 {
                for i3 in 1..=3 {
                    players[player] = p.incr(i1+i2+i3);

                    let ret = diracDice(nextPlayer, players, dp);
                    dp[idx] = (dp[idx].0 + 1 + ret.0, dp[idx].1 + 1 + ret.1);
                }
            }
        }
    }

    dp[idx]
}

fn main() {
//    std::io::stdin().read_to_string(&mut input).unwrap();
    let files = vec!["test", "data"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) =>
                input = inputs,
            Err(_) => continue,
        }

        let (pos1, pos2) : (usize, usize) = input.split_whitespace().map(|x| x.parse().unwrap()).collect_tuple().unwrap();

        println!("{} {}", pos1, pos2);

        let player1 = Player{pos:pos1-1, score:0};
        let player2 = Player{pos:pos2-1, score:0};

        let mut dp : Vec<(i64,i64)> = vec![(-27,-27); 2 * 10 * 10 * 21 * 21];
        let num1 = diracDice(0, [player1, player2], &mut dp);
        println!("{}\n{}\n", num1.0, num1.1);

    }
}
