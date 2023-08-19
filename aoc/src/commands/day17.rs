use camino::Utf8PathBuf;

use std::{collections::HashSet, env, error::Error, fmt, fs, io};

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
    log::debug!("left_disjoint({shifted_block:0>7b}, {old_block:0>7b})");
    disjoint(shifted_block << 1, old_block)
}

// determine if the right shifted new block is disjoint with old block
pub fn right_disjoint(shifted_block: u8, old_block: u8) -> bool {
    let x = disjoint(shifted_block >> 1, old_block);
    log::debug!("right_disjoint({shifted_block:0>7b}, {old_block:0>7b}) = {x}");
    x
}

pub fn next_left_wall(shift_block: u8) -> bool {
    (shift_block & 64_u8) > 0
}

// determine if the new block is disjoint with the old block
pub fn disjoint(this_block: u8, that_block: u8) -> bool {
    log::debug!(
        "disjoint(this_block: {this_block:0>7b}, that_block: {that_block:0>7b}) = {:?}",
        (this_block & that_block) == 0
    );
    (this_block & that_block) == 0
}

impl Rock {
    fn move_down(&mut self) {
        log::debug!("move_down");
        self.offset -= 1;
    }

    fn pop_front(&mut self) -> Option<u8> {
        if !self.row.is_empty() {
            self.offset += 1;
        }
        self.row.pop_front()
    }

    fn move_left(&mut self) {
        log::debug!("Rock.move_left");
        //self.row.iter_mut().map(|x| *x <<= 1).collect::<Vec<_>>();
        for x in self.row.iter_mut() {
            *x <<= 1;
        }
        //self.display();
    }

    fn move_right(&mut self) {
        log::debug!("Rock.move_right");
        //self.row.iter_mut().map(|x| *x >>= 1).collect::<Vec<_>>();
        for x in self.row.iter_mut() {
            *x >>= 1;
        }
        //self.display();
    }

    fn is_left_blocked(&self) -> bool {
        log::debug!("Rock.is_left_blocked");
        self.row.iter().any(|p| (p & 64_u8) > 0)
    }

    fn is_right_blocked(&self) -> bool {
        log::debug!("Rock.is_right_blocked");
        self.row.iter().any(|p| (p & 1_u8) > 0)
    }

    fn display(&self) {
        println!("Rock.display: {:?}", self.offset);
        for point in self.row.iter().rev() {
            println!("{point:0>7b}");
        }
        println!();
    }

    fn offset(&self) -> usize {
        log::debug!("Rock.offset: {:?}", self.offset);
        self.offset
    }

    fn row(&self, index: usize) -> u8 {
        log::debug!("Rock.row: {index} = {:0>7b}", self.row[index]);
        self.row[index]
    }

    fn has_layers(&self) -> bool {
        self.row.len() > 0
    }

    fn nlayers(&self) -> usize {
        log::debug!("Rock.nlayers: {:?}", self.row.len());
        self.row.len()
    }
}

#[derive(Debug)]
pub struct RockPile {
    rocks: VecDeque<u8>,
    offset: usize,
    nrocks: usize,
}

impl RockPile {
    fn new() -> RockPile {
        let rocks: VecDeque<u8> = VecDeque::from([0_u8]);
        RockPile { rocks, offset: 0, nrocks: 0 }
    }

    fn height(&self) -> usize {
        self.rocks.len() + self.offset
    }

    fn add_row(&mut self, row: usize, rock: u8) {
        log::debug!("RockPile.add_row: {row}");
        //self.display();
        self.rocks[row] = rock;
    }

    fn nlayers(&self) -> usize {
        log::debug!("RockPile.nlayers: {:?}", self.rocks.len());
        self.rocks.len()
    }

    fn is_blocked_below(&self, rock: &Rock) -> bool {
        log::debug!("RockPile.is_blocked_below");
        if rock.offset() == 0 {
            log::debug!("    rock.offset() == 0");
            return true;
        }
        if rock.offset() <= self.rocks.len() {
            let start: usize = rock.offset() - 1;
            let j: usize = 0;
            for i in start..self.rocks.len() {
                //let shifted_rock: u8 = rock.row(j) << 1;
                let curr_rock: u8 = rock.row(j);
                if !disjoint(curr_rock, self.rocks[i]) {
                    log::debug!("    !disjoint");
                    return true;
                }
            }
        }

        return false;
    }

    fn action_display(&self, rock: &Rock) {
        let vec: VecDeque<u8> = self.rocks.clone();
    }

    fn is_blocked_left(&self, rock: &Rock) -> bool {
        log::debug!("RockPile.is_blocked_left");
        if rock.is_left_blocked() {
            log::debug!("    rock.is_left_blocked() == true");
            //rock.display();
            return true;
        }
        if rock.offset() <= self.rocks.len() {
            let start: usize = rock.offset();
            let end: usize = rock.offset() + rock.nlayers();
            let end: usize = if end < self.nlayers() { end } else { self.nlayers() };
            let mut j: usize = 0;
            for i in start..end {
                if !disjoint(rock.row(j) << 1, self.rocks[i]) {
                    log::debug!("!disjoint(rock.row({j}))== true");
                    return true;
                }
                j += 1;
            }
        }

        log::debug!("is_blocked_left()== false");
        return false;
    }

    fn is_blocked_right(&self, rock: &Rock) -> bool {
        log::debug!("RockPile.is_blocked_right");
        if rock.is_right_blocked() {
            return true;
        }
        if rock.offset() <= self.rocks.len() {
            let start: usize = rock.offset();
            let end: usize = rock.offset() + rock.nlayers();
            let end: usize = if end < self.nlayers() { end } else { self.nlayers() };

            let mut j: usize = 0;
            for i in start..end {
                if !disjoint(rock.row(j) >> 1, self.rocks[i]) {
                    return true;
                }
                j += 1;
            }
        }

        return false;
    }

    fn add_rock_to_pile(&mut self, rock: &mut Rock) {
        log::debug!("RockPile.add_rock_to_pile");

        while rock.has_layers() {
            if rock.offset() < self.rocks.len() {
                let p = if let Some(p) = rock.pop_front() { p } else { todo!() };
                let i: usize = rock.offset() - 1;
                log::debug!("index: {i}, p: {p}, self.rocks[{i}]");
                self.rocks[i] = self.rocks[i] | p;
            } else {
                let p = if let Some(p) = rock.pop_front() { p } else { todo!() };
                self.rocks.push_back(p);
            }
        }
        self.nrocks += 1;
    }

    fn display(&self) {
        println!("RockPile::display: {:?}", self.offset);
        for point in self.rocks.iter().rev() {
            println!("{point:0>7b}");
        }
        println!();
    }

    fn collapse(&mut self, nlevels: usize) {
        log::debug!("RockPile::collapse: {nlevels}");
        assert!(self.rocks.len() > nlevels);
        self.rocks.drain(0..nlevels);
        self.offset += nlevels as usize;
    }

    fn new_rock(&mut self, rock_shape: RockShape) -> Box<Rock> {
        log::debug!("new_rock({:?})", rock_shape);
        rock_shape.projectile(self.nlayers() + ROCK_OFFSET)
    }

    fn rock_count(&self) -> usize {
        log::debug!("RockPile::rock_count");
        self.nrocks
    }
}

#[derive(Debug, EnumIter, PartialEq)]
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

fn parse_jets(input: &str) -> IResult<&str, Vec<char>> {
    let (input, vecs) = many1(one_of("><"))(input)?;
    Ok((input, vecs))
}

const NUM_ROCKS: usize = 3;
const ROCK_OFFSET: usize = 2;
//const NUM_ROCKS: usize = 1000000000000;

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
        let mut jetstream = jets.iter().cycle();
        let mut rock_pile = RockPile::new();

        for shape in RockShape::iter().cycle() {
            let mut rock = rock_pile.new_rock(shape);
            loop {
                let jet = jetstream.next();
                match *jet.unwrap() {
                    JetDirection::Left => {
                        if !rock_pile.is_blocked_left(&rock) {
                            println!("Jet of gas pushes rock left\n");
                            rock.move_left();
                        } else {
                            println!("Jet of gas pushes rock left, but nothing happens\n");
                        }
                    }
                    JetDirection::Right => {
                        if !rock_pile.is_blocked_right(&rock) {
                            println!("Jet of gas pushes rock right\n");
                            rock.move_right();
                        } else {
                            println!("Jet of gas pushes rock right, but nothing happens\n");
                        }
                    }
                    _ => {
                        log::debug!("panic");
                    }
                }
                if rock_pile.is_blocked_below(&rock) {
                    println!("Rock falls 1 unit, causing it to come to rest\n");
                    rock_pile.add_rock_to_pile(&mut rock);
                    rock_pile.display();
                    break;
                } else {
                    println!("Rock falls 1 unit\n");
                    rock.move_down();
                    rock.display();
                }
            }

            if rock_pile.rock_count() == NUM_ROCKS {
                break;
            }
        }

        let height: usize = rock_pile.height();
        println!("height = {height}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        rock_pile.add_row(0, 0u8);
        let rock = plank(2);
        assert!(!rock_pile.is_blocked_left(&rock), "Expect rockpile does not block rock",);

        //let rock = plank(1);
        //rock_pile.add_row(0, 63u8);
        //assert!(rock_pile.is_blocked_left(&rock), "Expect rockpile blocks rock",);
    }
}
