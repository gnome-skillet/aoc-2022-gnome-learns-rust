use camino::Utf8PathBuf;

use std::fs;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::collections::VecDeque;

use nom::{character::complete::one_of, multi::many1, *};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use env_logger::Env;

#[derive(Parser, Debug)]
pub struct Day17 {
    #[clap(long, short)]
    input: Utf8PathBuf,
}

#[derive(Debug)]
enum JetDirection {
    Right,
    Left,
    Down,
    Unknown,
}

impl JetDirection {
    #[allow(dead_code)]
    fn direction(c: char) -> JetDirection {
        match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            'v' => JetDirection::Down,
            _ => JetDirection::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Rock {
    row: VecDeque<u8>,
    offset: usize,
}

pub fn plank(offset: usize) -> Rock {
    let row: VecDeque<u8> = VecDeque::from([30_u8]);
    Rock { row, offset }
}

pub fn cross(offset: usize) -> Rock {
    let row: VecDeque<u8> = VecDeque::from([8_u8, 28_u8, 8_u8]);
    Rock { row, offset }
}

pub fn ell(offset: usize) -> Rock {
    let row: VecDeque<u8> = VecDeque::from([28_u8, 4_u8, 4_u8]);
    Rock { row, offset }
}

pub fn pole(offset: usize) -> Rock {
    let row: VecDeque<u8> = VecDeque::from([16_u8, 16_u8, 16_u8, 16_u8]);
    Rock { row, offset }
}

pub fn block(offset: usize) -> Rock {
    let row: VecDeque<u8> = VecDeque::from([24_u8, 24_u8]);
    Rock { row, offset }
}

// determine if the left shifted new block is disjoint with old block
pub fn left_disjoint(shifted_block: u8, old_block: u8) -> bool {
    disjoint(shifted_block << 1, old_block)
}

// determine if the right shifted new block is disjoint with old block
pub fn right_disjoint(shifted_block: u8, old_block: u8) -> bool {
    disjoint(shifted_block >> 1, old_block)
}

pub fn next_left_wall(shift_block: u8) -> bool {
    (shift_block & 64_u8) > 0
}

// determine if the new block is disjoint with the old block
pub fn disjoint(this_block: u8, that_block: u8) -> bool {
    (this_block & that_block) == 0
}

impl Rock {
    fn move_down(&mut self) {
        self.offset -= 1;
    }

    fn pop_front(&mut self) -> Option<u8> {
        if !self.row.is_empty() {
            self.offset += 1;
        }
        self.row.pop_front()
    }

    fn move_left(&mut self) {
        for x in self.row.iter_mut() {
            *x <<= 1;
        }
    }

    fn move_right(&mut self) {
        for x in self.row.iter_mut() {
            *x >>= 1;
        }
    }

    fn is_left_blocked(&self) -> bool {
        self.row.iter().any(|p| (p & 64_u8) > 0)
    }

    fn is_right_blocked(&self) -> bool {
        self.row.iter().any(|p| (p & 1_u8) > 0)
    }

    #[allow(dead_code)]
    fn display(&self) {
        let string =
            self.row.iter().rev().map(|x| format!("{x:07b}")).collect::<Vec<String>>().join("\n");
        let string = string.replace("1", "x");
        let string = string.replace("0", ".");
        println!("{}", string);
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn row(&self, index: usize) -> u8 {
        self.row[index]
    }

    fn has_layers(&self) -> bool {
        !self.row.is_empty()
    }

    fn nlayers(&self) -> usize {
        self.row.len()
    }

    fn height(&self) -> usize {
        self.row.len() + self.offset
    }
}

pub struct RockPile {
    rocks: VecDeque<u8>,
    offset: usize,
    nrocks: usize,
}

impl RockPile {
    fn new() -> RockPile {
        let rocks: VecDeque<u8> = VecDeque::new();
        RockPile { rocks, offset: 0, nrocks: 0 }
    }

    #[allow(dead_code)]
    fn height(&self) -> usize {
        self.rocks.len() + self.offset
    }

    #[allow(dead_code)]
    fn add_row(&mut self, row: usize, rock: u8) {
        self.rocks[row] = rock;
    }

    fn nlayers(&self) -> usize {
        self.rocks.len()
    }

    fn is_blocked_below(&self, rock: &Rock) -> bool {
        if rock.offset() == 0 {
            return true;
        }
        if rock.offset() <= self.rocks.len() {
            let start: usize = rock.offset() - 1;
            for (i, j) in (start..self.rocks.len()).enumerate() {
                if i >= rock.nlayers() {
                    break;
                }
                let curr_rock: u8 = rock.row(i);
                if !disjoint(curr_rock, self.rocks[j]) {
                    return true;
                }
            }
        }

        false
    }

    fn is_blocked_left(&self, rock: &Rock) -> bool {
        // check if blocked by left wall
        if rock.is_left_blocked() {
            return true;
        }
        if rock.offset() <= self.rocks.len() {
            let start: usize = rock.offset();
            let end: usize = rock.offset() + rock.nlayers();
            let end: usize = if end < self.nlayers() { end } else { self.nlayers() };
            for (j, i) in (start..end).enumerate() {
                if !disjoint(rock.row(j) << 1, self.rocks[i]) {
                    return true;
                }
            }
        }

        false
    }

    fn is_blocked_right(&self, rock: &Rock) -> bool {
        // check if blocked by right wall
        if rock.is_right_blocked() {
            return true;
        }
        if rock.offset() <= self.rocks.len() {
            let start: usize = rock.offset();
            let end: usize = rock.offset() + rock.nlayers();
            let end: usize = if end < self.nlayers() { end } else { self.nlayers() };

            for (j, i) in (start..end).enumerate() {
                if !disjoint(rock.row(j) >> 1, self.rocks[i]) {
                    return true;
                }
            }
        }

        false
    }

    fn add_rock_to_pile(&mut self, rock: &mut Rock) {
        while rock.has_layers() {
            if rock.offset() < self.rocks.len() {
                let p = if let Some(p) = rock.pop_front() { p } else { todo!() };
                let i: usize = rock.offset() - 1;
                self.rocks[i] |= p;
            } else {
                let p = if let Some(p) = rock.pop_front() { p } else { todo!() };
                self.rocks.push_back(p);
            }
        }
        self.nrocks += 1;
    }

    #[allow(dead_code)]
    fn display(&self) {
        let string =
            self.rocks.iter().rev().map(|x| format!("{x:07b}")).collect::<Vec<String>>().join("\n");
        let string = string.replace("1", "#");
        let string = string.replace("0", ".");
        let nrocks: usize = self.nrocks;
        let height: usize = self.nlayers();
        let msg = format!("After {nrocks} rocks, the tower of rocks will be {height} units tall");
        println!("{}\n{}", msg, string);
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!("{:?}", self.rocks);
    }

    fn build_pile(&mut self, jets: Vec<JetDirection>, nrocks: usize, debug: bool) {
        let mut jetstream = jets.iter().cycle();
        let mut x: usize = 0;
        for shape in RockShape::iter().cycle() {
            let mut rock = self.new_rock(shape);
            x += 1;
            let mut steps: usize = 1;
            loop {
                let jet = jetstream.next();
                match *jet.unwrap() {
                    JetDirection::Left => {
                        if debug {
                            print!("{x},{steps},left,");
                        }
                        if !self.is_blocked_left(&rock) {
                            if debug {
                                print!("unblocked,");
                            }
                            rock.move_left();
                        } else {
                            if debug {
                                if debug {
                                    print!("blocked,");
                                }
                            }
                        }
                    }
                    JetDirection::Right => {
                        if debug {
                            print!("{x},{steps},right,");
                        }
                        if !self.is_blocked_right(&rock) {
                            if debug {
                                print!("unblocked,");
                            }
                            rock.move_right();
                        } else {
                            if debug {
                                print!("blocked,");
                            }
                        }
                    }
                    _ => {
                        log::debug!("panic");
                    }
                }
                if debug {
                    print!("down,");
                }
                if self.is_blocked_below(&rock) {
                    self.add_rock_to_pile(&mut rock);
                    if debug {
                        println!("blocked,{:?}", self.nlayers());
                    }
                    break;
                } else {
                    rock.move_down();
                    if debug {
                        println!("unblocked,{:?}", self.nlayers());
                    }
                }
                steps += 1;
            }
            if self.rock_count() == 50 {
                //println!("{steps} mod {MAX_CYCLE} == {:?}", steps.rem_euclid(MAX_CYCLE));
            }
            if self.rock_count().rem_euclid(MAX_CYCLE) == 0 {
                //println!("hit {MAX_CYCLE} at {steps}");
                if let Some(index) = self.highest_plateau() {
                    // println!("collapse at {index}");
                    self.collapse(index);
                }
            }
            if self.rock_count() == nrocks {
                break;
            }
        }
    }

    fn highest_plateau(&mut self) -> Option<usize> {
        if let Some(index) = self.rocks.iter().rev().position(|c| *c == 127_u8) {
            return Some(self.rocks.len() - index);
        }
        None
    }

    fn collapse(&mut self, nlevels: usize) {
        log::debug!("RockPile::collapse: {nlevels}");
        assert!(self.rocks.len() > nlevels);
        self.rocks.drain(0..nlevels);
        self.offset += nlevels as usize;
    }

    fn new_rock(&mut self, rock_shape: RockShape) -> Box<Rock> {
        //println!("starting position (2, {:?})", self.nlayers() + ROCK_OFFSET);
        rock_shape.projectile(self.nlayers() + ROCK_OFFSET)
    }

    fn rock_count(&self) -> usize {
        self.nrocks
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum RockShape {
    Plank,
    Cross,
    Ell,
    Pole,
    Block,
}

impl RockShape {
    pub fn projectile(&self, start: usize) -> Box<Rock> {
        match self {
            RockShape::Plank => Box::new(plank(start)),
            RockShape::Cross => Box::new(cross(start)),
            RockShape::Ell => Box::new(ell(start)),
            RockShape::Pole => Box::new(pole(start)),
            RockShape::Block => Box::new(block(start)),
        }
    }
}

#[allow(dead_code)]
fn parse_jets(input: &str) -> IResult<&str, Vec<char>> {
    let (input, vecs) = many1(one_of("><"))(input)?;
    Ok((input, vecs))
}

#[allow(dead_code)]
const ROCK_OFFSET: usize = 3;
const MAX_CYCLE: usize = 100000;
//const NUM_ROCKS: usize = 1000000000000;

#[allow(dead_code)]
fn get_jetstream(path: &Utf8PathBuf) -> Vec<JetDirection> {
    let characters: String = fs::read_to_string(path).unwrap();
    let (_, jets) = parse_jets(&characters).unwrap();
    jets.into_iter().map(JetDirection::direction).collect()
}

impl CommandImpl for Day17 {
    fn main(&self) -> Result<(), DynError> {
        let env = Env::default()
            .filter_or("MY_LOG_LEVEL", "warn")
            .write_style_or("MY_LOG_STYLE", "always");
        env_logger::init_from_env(env);

        let jets: Vec<JetDirection> = get_jetstream(&self.input);
        let mut rock_pile = RockPile::new();
        //rock_pile.build_pile(jets, 2022, false);
        rock_pile.build_pile(jets, 10000000, false);
        //rock_pile.build_pile(jets, 2022, false);
        println!("the rock pile is {:?} tall", rock_pile.height());
        //rock_pile.dump();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_pile_works() {
        let jets: Vec<JetDirection> = vec![
            JetDirection::Left,
            JetDirection::Left,
            JetDirection::Left,
            JetDirection::Left,
            JetDirection::Right,
            JetDirection::Right,
            JetDirection::Right,
            JetDirection::Right,
        ];
        let mut rock_pile = RockPile::new();
        rock_pile.build_pile(jets, 2, false);

        //let jets: Vec<JetDirection> = vec![
        //    JetDirection::Left,
        //    JetDirection::Left,
        //    JetDirection::Left,
        //    JetDirection::Left,
        //    JetDirection::Right,
        //    JetDirection::Right,
        //    JetDirection::Right,
        //    JetDirection::Right,
        //];
        //let mut rock_pile = RockPile::new();
        //rock_pile.build_pile(jets, 3, false);
    }

    #[test]
    fn disjoint_works() {
        let new_block = 1_u8;
        let old_block = 63_u8;
        assert!(
            !disjoint(new_block, old_block),
            "Expect {new_block:0>7b} and {old_block:0>7b} are not disjoint",
        );
    }

    #[test]
    fn left_disjoint_works() {
        let shifted_block = 1;
        let old_block = 2;
        assert!(
            !left_disjoint(shifted_block, old_block),
            "Expect {shifted_block:0>7b} and {old_block:0>7b} are not left disjoint",
        );
        let shifted_block = 1;
        let old_block = 4;
        assert!(
            left_disjoint(shifted_block, old_block),
            "Expect {shifted_block:0>7b} and {old_block:0>7b} are left disjoint",
        );
    }

    #[test]
    fn right_disjoint_works() {
        let shifted_block = 2;
        let old_block = 1;
        assert!(
            !right_disjoint(shifted_block, old_block),
            "Expect {shifted_block:0>7b} and {old_block:0>7b} are not right disjoint",
        );
        let shifted_block = 4;
        let old_block = 1;
        assert!(
            right_disjoint(shifted_block, old_block),
            "Expect {shifted_block:0>7b} and {old_block:0>7b} are right disjoint",
        );
    }

    #[test]
    fn rockpile_block_works() {
        let mut rock_pile = RockPile::new();
        let rock = plank(2);
        assert!(!rock_pile.is_blocked_left(&rock), "Expect rockpile does not block rock",);

        //let rock = plank(1);
        //rock_pile.add_row(0, 63u8);
        //assert!(rock_pile.is_blocked_left(&rock), "Expect rockpile blocks rock",);
    }
}
