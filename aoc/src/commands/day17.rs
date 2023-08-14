use camino::Utf8PathBuf;

use std::{collections::HashSet, error::Error, fmt, fs, io};

use clap::Parser;

use super::{CommandImpl, DynError};

use nom::{character::complete::one_of, multi::many1, *};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
                            //
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
    row: Vec<u8>,
    offset: usize,
}

pub fn plank(offset: usize) -> Rock {
    Rock { row: vec![30_u8], offset }
}

pub fn cross(offset: usize) -> Rock {
    Rock { row: vec![8_u8, 28_u8, 8_u8], offset }
}

pub fn ell(offset: usize) -> Rock {
    Rock { row: vec![28_u8, 4_u8, 4_u8], offset }
}

pub fn pole(offset: usize) -> Rock {
    Rock { row: vec![16_u8, 16_u8, 16_u8, 16_u8], offset }
}

pub fn block(offset: usize) -> Rock {
    Rock { row: vec![24_u8, 24_u8], offset }
}

// determine if the left shifted new block is disjoint with old block
pub fn left_disjoint(shifted_block: u8, old_block: u8) -> bool {
    disjoint(shifted_block << 1, old_block)
}

// determine if the right shifted new block is disjoint with old block
pub fn right_disjoint(shifted_block: u8, old_block: u8) -> bool {
    disjoint(shifted_block >> 1, old_block)
}

// determine if the new block is disjoint with the old block
pub fn disjoint(this_block: u8, that_block: u8) -> bool {
    this_block & that_block == 0
}

// combine shifted_block and old_block
pub fn combine(shifted_block: usize, old_block: usize) -> usize {
    shifted_block | old_block
}

impl Rock {
    fn move_down(&mut self) {
        self.offset -= 1;
    }

    fn move_left(&mut self) {
        if self.can_move_left() {
            //self.row.iter_mut().map(|x| *x <<= 1).collect::<Vec<_>>();
            for x in self.row.iter_mut() {
                *x <<= 1;
            }
        }
    }

    fn move_right(&mut self) {
        if self.can_move_right() {
            //self.row.iter_mut().map(|x| *x >>= 1).collect::<Vec<_>>();
            for x in self.row.iter_mut() {
                *x >>= 1;
            }
        }
    }

    fn has_hit_bottom(&self) -> bool {
        self.offset > 0
    }

    fn can_move_left(&self) -> bool {
        self.row.iter().all(|p| p & 64u8 == 0)
    }

    fn can_move_right(&self) -> bool {
        self.row.iter().all(|p| p & 1u8 == 0)
    }

    fn highest_point(&self) -> usize {
        self.offset + self.row.len() as usize - 1
    }

    fn lowest_point(&self) -> usize {
        self.offset as usize
    }

    fn shape(&self) -> &'static str {
        //format!("rock {:?}", self.row).as_str()
        "rock"
    }

    fn display(&self) {
        println!("offset: {:?}", self.offset);
        for point in self.row.iter().rev() {
            println!("{point:0>7b}");
        }
        println!();
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn row(&self, row_index: usize) -> u8 {
        println!("row: {row_index} - {:?}", self.offset);
        let index = (row_index - self.offset);
        self.row[index as usize]
    }
}

#[derive(Debug)]
pub struct RockPile {
    rocks: Vec<u8>,
    offset: usize,
    nrocks: usize,
}

impl RockPile {
    fn new() -> RockPile {
        RockPile { rocks: vec![255u8, 1], offset: 0, nrocks: 0 }
    }

    fn add_row(&mut self, row: usize, rock: u8) {
        self.rocks[row] = rock;
    }

    // index of tallest rock above floor
    fn highest_point(&self) -> usize {
        if self.rocks.len() > 0 {
            self.offset
        } else {
            self.offset + self.rocks.len() - 1
        }
    }

    fn is_blocked(&self, rock: &Rock) -> bool {
        if rock.offset() > (self.highest_point() + 1) {
            return false;
        }
        let rock_row: u8 = rock.row(1);
        let rockpile_row: u8 = self.rocks[0];
        //let highest_row: Vec<usize> = vec![self.highest_point(), rock.highest_point()];
        //let lowest_high_point = highest_row.iter().min().unwrap();
        //let j = 0;
        //for i in rock.offset()..*lowest_high_point {}
        //rock.row.iter().any(|p| self.rocks.contains(p))
        !disjoint(rock_row, rockpile_row)
    }

    fn is_open(&self, rock: &Rock) -> bool {
        !self.is_blocked(rock)
    }

    fn display(&self) {
        for point in self.rocks.iter().rev() {
            println!("{point:0>7b}");
        }
        println!();
    }

    //fn is_covered(&mut self, row: usize) -> bool {
    //(0..7).into_iter().map(|x| (x, row)).all(|p| self.rocks.contains(&p))
    //    todo!()
    //}

    fn collapse(&mut self, nlevels: usize) {
        assert!(self.rocks.len() > nlevels);
        self.rocks.drain(0..nlevels);
        self.offset += nlevels as usize;
    }

    fn new_rock(&mut self, rock_shape: RockShape) -> Box<Rock> {
        println!("new_rock({:?})", rock_shape);
        rock_shape.projectile(self.highest_point() + 3)
    }

    fn add_rock_to_pile(&mut self, rock: &Rock) {
        self.nrocks += 1;
    }

    fn rock_count(&self) -> usize {
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

const NUM_ROCKS: usize = 5;
const ROCK_OFFSET: usize = 3;
//const NUM_ROCKS: usize = 2022;
//const NUM_ROCKS: usize = 1000000000000;

fn get_jetstream(path: &Utf8PathBuf) -> Vec<JetDirection> {
    let characters: String = fs::read_to_string(path).unwrap();
    let (_, jets) = parse_jets(&characters).unwrap();
    jets.into_iter().map(JetDirection::direction).collect()
}

impl CommandImpl for Day17 {
    fn main(&self) -> Result<(), DynError> {
        let jets: Vec<JetDirection> = get_jetstream(&self.input);
        let mut jetstream = jets.iter().cycle();

        let mut rock_pile = RockPile::new();

        for shape in RockShape::iter().cycle() {
            let mut rock = rock_pile.new_rock(shape);
            println!("init");
            rock.display();
            loop {
                let jet = jetstream.next();
                match *jet.unwrap() {
                    JetDirection::Left => {
                        println!("left");
                        rock.move_left();
                    }
                    JetDirection::Right => {
                        println!("right");
                        rock.move_right();
                    }
                    _ => println!("panic"),
                }
                rock.display();
                if !rock.has_hit_bottom() {
                    rock.move_down();
                } else {
                    rock_pile.add_rock_to_pile(&rock);
                    break;
                }
            }

            if rock_pile.rock_count() == NUM_ROCKS {
                break;
            }
        }
        println!("top {:?}", rock_pile.highest_point());
        let mut rock_pile = RockPile::new();
        rock_pile.add_row(0, 63u8);
        let rock = plank(1);
        rock.display();
        rock_pile.display();
        let blocked: bool = rock_pile.is_blocked(&rock);
        println!("blocked = {blocked}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_works() {
        let shifted_block = 1;
        let old_block = 2;
        assert_eq!(
            combine(shifted_block, old_block),
            3,
            "Expect {shifted_block:0>7b} | {old_block:0>7b} = {:0>7b}",
            3
        );
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
        rock_pile.add_row(0, 0u8);
        let rock = plank(2);
        assert!(!rock_pile.is_blocked(&rock), "Expect rockpile does not block rock",);

        let rock = plank(1);
        rock_pile.add_row(0, 63u8);
        assert!(rock_pile.is_blocked(&rock), "Expect rockpile blocks rock",);
    }
}
