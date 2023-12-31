use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

const GROUND: char = '.';
const START: char = 'S';
const VERTICAL: char = '|';
const HORIZONTAL: char = '-';
const TOP_RIGHT: char = 'L';
const TOP_LEFT: char = 'J';
const BOTTOM_RIGHT: char = 'F';
const BOTTOM_LEFT: char = '7';

const DOWN: Pair = Pair {
    first: 1,
    second: 0,
};
const UP: Pair = Pair {
    first: -1,
    second: 0,
};

const LEFT: Pair = Pair {
    first: 0,
    second: -1,
};
const RIGHT: Pair = Pair {
    first: 0,
    second: 1,
};

fn possible_dirs(c: char) -> (&'static Pair, &'static Pair) {
    match c {
        VERTICAL => (&UP, &DOWN),
        HORIZONTAL => (&LEFT, &RIGHT),
        TOP_LEFT => (&LEFT, &UP),
        TOP_RIGHT => (&RIGHT, &UP),
        BOTTOM_LEFT => (&LEFT, &DOWN),
        BOTTOM_RIGHT => (&RIGHT, &DOWN),
        _ => {
            assert!(false);
            (&UP, &UP)
        }
    }
}

fn next_dir(c: char, in_dir: &Pair) -> &Pair {
    //println!("{} {}", c, in_dir.print_dir());
    match in_dir {
        &DOWN => match c {
            VERTICAL => return &DOWN,
            TOP_RIGHT => return &RIGHT,
            TOP_LEFT => return &LEFT,
            _ => assert!(false),
        },
        &UP => match c {
            VERTICAL => return &UP,
            BOTTOM_RIGHT => return &RIGHT,
            BOTTOM_LEFT => return &LEFT,
            _ => assert!(false),
        },
        &RIGHT => match c {
            HORIZONTAL => return &RIGHT,
            BOTTOM_LEFT => return &DOWN,
            TOP_LEFT => return &UP,
            _ => assert!(false),
        },
        &LEFT => match c {
            HORIZONTAL => return &LEFT,
            BOTTOM_RIGHT => return &DOWN,
            TOP_RIGHT => return &UP,
            _ => assert!(false),
        },
        _ => assert!(false),
    }

    assert!(false);
    &DOWN
}

fn goes_to(c: char, dir: &Pair) -> bool {
    match dir {
        &RIGHT => return c == HORIZONTAL || c == TOP_RIGHT || c == BOTTOM_RIGHT,
        &LEFT => return c == HORIZONTAL || c == TOP_LEFT || c == BOTTOM_LEFT,
        &UP => return c == VERTICAL || c == TOP_LEFT || c == TOP_RIGHT,
        &DOWN => return c == VERTICAL || c == BOTTOM_LEFT || c == BOTTOM_RIGHT,
        _ => {
            assert!(false);
            false
        }
    }
}

fn get_connection(l: char, r: char, t: char, b: char) -> char {
    if goes_to(l, &RIGHT) {
        if goes_to(r, &LEFT) {
            return HORIZONTAL;
        }
        if goes_to(t, &DOWN) {
            return TOP_LEFT;
        }
        if goes_to(b, &UP) {
            return BOTTOM_LEFT;
        }
        assert!(false);
    }

    if goes_to(r, &LEFT) {
        if goes_to(t, &DOWN) {
            return TOP_RIGHT;
        }
        if goes_to(b, &UP) {
            return BOTTOM_RIGHT;
        }
        assert!(false);
    }

    if goes_to(t, &DOWN) {
        if goes_to(b, &UP) {
            return VERTICAL;
        }
        assert!(false);
    }
    return GROUND;
}

fn read_inputs(filename: String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    first: isize,
    second: isize,
}

impl Pair {
    fn new(first: isize, second: isize) -> Pair {
        Pair {
            first: first,
            second: second,
        }
    }
    fn copy(other: &Pair) -> Pair {
        Pair {
            first: other.first,
            second: other.second,
        }
    }
    fn add(&self, other: &Pair) -> Pair {
        Pair {
            first: self.first + other.first,
            second: self.second + other.second,
        }
    }

    fn print_dir(&self) -> char {
        match self {
            &UP => '^',
            &DOWN => 'v',
            &RIGHT => '>',
            &LEFT => '<',
            _ => '?',
        }
    }
}

#[derive(Debug)]
struct Board {
    start: Pair,
    board: Vec<Vec<char>>,
    path: Vec<Vec<i32>>,
}

impl Board {
    fn new(_board: &Vec<Vec<char>>) -> Board {
        let start = find_start(_board).add(&Pair::new(1, 1));
        let mut board: Vec<Vec<char>> = vec![];

        let width = _board[0].len();

        board.push(vec![GROUND; width + 2]);
        for ri in 0.._board.len() {
            board.push(vec![GROUND; width + 2]);
            for ci in 0.._board[ri].len() {
                board[ri + 1][ci + 1] = _board[ri][ci];
            }
        }
        board.push(vec![GROUND; width + 2]);

        let mut ret = Board {
            start: Pair::copy(&start),
            board: board,
            path: vec![vec![i32::MAX; width + 2]; _board.len() + 2],
        };
        ret.set(
            &start,
            get_connection(
                ret.at(&start.add(&LEFT)),
                ret.at(&start.add(&RIGHT)),
                ret.at(&start.add(&UP)),
                ret.at(&start.add(&DOWN)),
            ),
        );
        ret
    }

    fn set(&mut self, pos: &Pair, c: char) {
        self.board[pos.first as usize][pos.second as usize] = c;
    }

    fn steps_set(&mut self, pos: &Pair, s: i32) {
        self.path[pos.first as usize][pos.second as usize] = s;
    }

    fn at(&self, pos: &Pair) -> char {
        return self.board[pos.first as usize][pos.second as usize];
    }

    fn steps_at(&self, pos: &Pair) -> i32 {
        return self.path[pos.first as usize][pos.second as usize];
    }

    fn follow(&mut self, pos: &Pair, in_dir: &Pair, steps: i32) -> i32 {
        if self.steps_at(pos) <= steps {
            return self.steps_at(pos);
        }
        self.steps_set(pos, steps);
        let next_dir = next_dir(self.at(pos), in_dir);
        let next_pos = pos.add(next_dir);
        self.follow(&next_pos, next_dir, steps + 1)
    }

    fn closest_dst(&mut self) -> i32 {
        let node = Pair::copy(&self.start);
        self.steps_set(&node, 0);
        let dirs = possible_dirs(self.at(&node));
        self.follow(&node.add(dirs.0), dirs.0, 1);
        self.follow(&node.add(dirs.1), dirs.1, 1)
    }

    fn num_enclosed(&mut self) -> i64 {
        let mut num = 0;
        for ri in 0..self.board.len() {
            let mut inside = false;
            let mut last_crossing = VERTICAL;
            for ci in 0..self.board[ri].len() {
                let mut tile = self.board[ri][ci];
                let s = self.path[ri][ci];
                // part of loop
                if s != i32::MAX {
                    //is crossing
                    if goes_to(tile, &UP) || goes_to(tile, &DOWN) {
                        if tile == VERTICAL {
                            inside = !inside;
                        } else if last_crossing != VERTICAL {
                            if (goes_to(tile, &UP) && goes_to(last_crossing, &DOWN))
                                || (goes_to(tile, &DOWN) && goes_to(last_crossing, &UP))
                            {
                                inside = !inside;
                            }
                            tile = VERTICAL;
                        }
                        last_crossing = tile;
                    }
                } else if inside {
                    num += 1;
                    self.path[ri][ci] = -1;
                }
            }
        }
        num
    }

    fn print(&self, pos: Pair) {
        for ri in 0..self.board.len() {
            for ci in 0..self.board[ri].len() {
                if pos.first == ri as isize && pos.second == ci as isize {
                    print!("*");
                } else {
                    if self.path[ri][ci] < 0 {
                        print!("▓");
                    } else if self.path[ri][ci] == i32::MAX {
                        print!("-");
                        //print!("░");
                    } else {
                        let c = match self.board[ri][ci] {
                            TOP_RIGHT => '╚',
                            BOTTOM_RIGHT => '╔',
                            VERTICAL => '║',
                            HORIZONTAL => '═',
                            TOP_LEFT => '╝',
                            BOTTOM_LEFT => '╗',
                            _ => self.board[ri][ci],
                        };
                        print!("{}", c);
                    }
                }
            }
            println!();
        }
    }
}

fn find_start(board: &Vec<Vec<char>>) -> Pair {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] == START {
                return Pair::new(row as isize, col as isize);
            }
        }
    }
    //should not happen
    Pair::new(0, 0)
}

fn main() {
    let files = vec!["input"];
    for file in files {
        let input: String;
        match read_inputs(file.to_string()) {
            Ok(inputs) => input = inputs,
            Err(_) => process::exit(0),
        }
        let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut board = Board::new(&input);
        println!("star1: {}", board.closest_dst());
        println!("star2: {}", board.num_enclosed());
        //board.print(Pair::new(-1, -1));
    }
}
