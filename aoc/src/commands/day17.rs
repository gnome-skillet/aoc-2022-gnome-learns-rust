use camino::Utf8PathBuf;

use std::fs;

use clap::Parser;

use super::{CommandImpl, DynError};

use std::collections::{HashMap, VecDeque};

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

    fn idx(&self) -> String {
        match *self {
            JetDirection::Left => "Left".to_string(),
            JetDirection::Right => "Right".to_string(),
            JetDirection::Down => "Down".to_string(),
            _ => "Unknown".to_string(),
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

    fn top_of_pile(&self) -> Option<u64> {

        // If the chamber is less than 8 levels tall, we can't take a skyline
        if self.nlayers() < 8 {
            return None;
        }

        // Take the top 8 levels of the chamber and fold them into a 64-bit integer.
        let result = self
            .rocks
            .iter()
            .rev()
            .take(8)
            .fold(0u64, |acc, byte| (acc << 8) | *byte as u64);
        Some(result)
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

    fn add_rocks_to_pile(&mut self, jets: Vec<JetDirection>, total_rocks: usize) {
        let mut jetstream = jets.iter().cycle();
        let rocks = RockShape::iter().cycle();
        let mut visited = HashMap::with_capacity(2048);
        for shape in rocks {
            let mut rock = self.new_rock(&shape);
            loop {
                let jet = jetstream.next();
                match *jet.unwrap() {
                    JetDirection::Left => {
                        if !self.is_blocked_left(&rock) {
                            rock.move_left();
                        }
                    }
                    JetDirection::Right => {
                        if !self.is_blocked_right(&rock) {
                            rock.move_right();
                        }
                    }
                    _ => {
                        log::debug!("panic");
                    }
                }
                if self.is_blocked_below(&rock) {
                    self.add_rock_to_pile(&mut rock);
                    if let Some(top) = self.top_of_pile() {
                        let stage = (top, shape.idx(), jet.expect("expect string").idx());
                        if let Some((prev_rocks, prev_height)) = visited.get(&stage) {
                            // number of rocks added in each repeating cycle.
                            let repeat_len: usize = self.nrocks - prev_rocks;

                            // The number of repeats left before we add the final rock.
                            let repeats: usize = (total_rocks - self.nrocks) / repeat_len;

                            // Add all the rocks in all the repeating cycles between here and the end.
                            //println!("prev_rocks: {prev_rocks}, total_rocks: {total_rocks}, repeat_len: {repeat_len}, repeats: {repeats}");
                            self.nrocks += repeat_len * repeats;

                            // Add the chamber height of the cycle to the accumulated height
                            //println!("repeats: {repeats}, height{:?}, prev_height: {prev_height}", self.height());
                            //println!("add {:?} height", repeats * (self.height() - prev_height));
                            self.offset += repeats * (self.height() - prev_height);

                            // Clear the map of visited states. No need to repeat
                            visited.clear();
                        } else {
                            visited.insert(stage, (self.nrocks, self.height()));
                        }
                    }
                    break;
                } else {
                    rock.move_down();
                }
            }
            if self.nrocks == total_rocks {
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

    #[allow(dead_code)]
    fn collapse(&mut self, nlevels: usize) {
        log::debug!("RockPile::collapse: {nlevels}");
        assert!(self.rocks.len() > nlevels);
        self.rocks.drain(0..nlevels);
        self.offset += nlevels as usize;
    }

    fn new_rock(&mut self, rock_shape: &RockShape) -> Box<Rock> {
        //println!("starting position (2, {:?})", self.nlayers() + ROCK_OFFSET);
        rock_shape.projectile(self.nlayers() + ROCK_OFFSET)
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

    fn idx(&self) -> String {
        match *self {
            RockShape::Plank => "Plank".to_string(),
            RockShape::Cross => "Cross".to_string(),
            RockShape::Ell => "Ell".to_string(),
            RockShape::Pole => "Pole".to_string(),
            RockShape::Block => "Block".to_string(),
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
//const MAX_CYCLE: usize = 100000;
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
            .write_style_or("MY_LOG_S&TYLE", "always");
        env_logger::init_from_env(env);

        let jets: Vec<JetDirection> = get_jetstream(&self.input);
        let mut rock_pile = RockPile::new();
        rock_pile.add_rocks_to_pile(jets, 1_000_000_000_000);
        println!("the rock pile is {:?} tall", rock_pile.height());
        let jets: Vec<JetDirection> = get_jetstream(&self.input);
        let mut rock_pile = RockPile::new();
        rock_pile.add_rocks_to_pile(jets, 2022);
        println!("the rock pile is {:?} tall", rock_pile.height());

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
        rock_pile.add_rocks_to_pile(jets, 2, false);

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
        //rock_pile.add_rocks_to_pile(jets, 3, false);
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
