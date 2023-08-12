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

pub trait Projectile {
    fn can_move_down(&self) -> bool;
    fn can_move_left(&self) -> bool;
    fn can_move_right(&self) -> bool;
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn lowest_point(&self) -> u64;
    fn highest_point(&self) -> u64;
    fn shape(&self) -> &'static str;
    fn display(&self);
}

#[derive(Debug)]
pub struct Rock {
    points: Vec<u8>,
    offset: u64,
}

pub fn plank(offset: u64) -> Rock {
    //println!("plank({:?})", start);
    Rock { points: vec![30 as u8], offset: offset }
}

pub fn cross(offset: u64) -> Rock {
    //println!("cross({:?})", start);
    Rock { points: vec![8u8, 28u8, 8u8], offset }
}

pub fn ell(offset: u64) -> Rock {
    //println!("ell({:?})", start);
    Rock { points: vec![28u8, 4u8, 4u8], offset }
}

pub fn pole(offset: u64) -> Rock {
    //println!("pole({:?})", start);
    Rock { points: vec![16u8, 16u8, 16u8, 16u8], offset }
}

pub fn block(offset: u64) -> Rock {
    //println!("block({:?})", start);
    Rock { points: vec![24u8, 24u8], offset }
}

impl Projectile for Rock {
    fn move_down(&mut self) {
        self.offset -= 1;
    }

    fn move_left(&mut self) {
        if self.can_move_left() {
            self.points.iter_mut().map(|x| *x <<= 1).collect::<Vec<_>>();
        }
    }

    fn move_right(&mut self) {
        self.points.iter_mut().map(|x| *x >>= 1).collect::<Vec<_>>();
    }

    fn can_move_down(&self) -> bool {
        self.offset > 0
    }

    fn can_move_left(&self) -> bool {
        self.points.iter().all(|p| p & 64u8 == 0)
    }

    fn can_move_right(&self) -> bool {
        self.points.iter().all(|p| p & 1u8 == 0)
    }

    fn highest_point(&self) -> u64 {
        self.offset + self.points.len() as u64
    }

    fn lowest_point(&self) -> u64 {
        self.offset as u64
    }

    fn shape(&self) -> &'static str {
        //format!("rock {:?}", self.points).as_str()
        "rock"
    }

    fn display(&self) {
        for point in self.points.iter().rev() {
            println!("{point:0>7b}");
        }
        println!("");
    }
}

#[derive(Debug)]
pub struct RockPile {
    rocks: Vec<u8>,
    offset: u64,
}

impl RockPile {
    fn new() -> RockPile {
        RockPile { rocks: vec![255u8, 1], offset: 0 }
    }

    // index of first rock formation above floor
    fn tallest_point(&self) -> u64 {
        self.offset + self.rocks.len() as u64 - 1
    }

    fn is_blocked(&self, rock: &Rock) -> bool {
        rock.points.iter().any(|p| self.rocks.contains(p))
    }

    fn is_open(&self, rock: &Rock) -> bool {
        !self.is_blocked(rock)
    }

    fn display(&self) {
        for point in self.rocks.iter().rev() {
            println!("{point:0>7b}");
        }
        println!("");
    }

    fn is_covered(&mut self, row: u64) -> bool {
        //(0..7).into_iter().map(|x| (x, row)).all(|p| self.rocks.contains(&p))
        todo!()
    }

    fn collapse(&mut self, nlevels: usize) {
        assert!(self.rocks.len() > nlevels);
        self.rocks.drain(0..nlevels);
        self.offset += nlevels as u64;
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
    pub fn projectile(&self, start: u64) -> Box<dyn Projectile> {
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

//const NUM_ROCKS: u64 = 2022;
const NUM_ROCKS: u64 = 5;
const ROCK_OFFSET: u64 = 4;
//const NUM_ROCKS: u64 = 1000000000000;
//const NUM_ROCKS: usize = 2;

impl CommandImpl for Day17 {
    fn main(&self) -> Result<(), DynError> {
        let characters: String = fs::read_to_string(&self.input).unwrap();
        let (_, jets) = parse_jets(&characters).unwrap();
        let jets: Vec<JetDirection> =
            jets.into_iter().map(|x| JetDirection::direction(x)).collect();
        let mut jet_iter = jets.iter().cycle();

        let mut n: u64 = 0;
        let mut rock_pile = RockPile::new();
        //println!("rock_pile: {:?}", rock_pile);
        let N_JETS: u64 = jets.len() as u64;
        const N_SHAPES: u64 = 5;
        println!("njets = {N_JETS}, n_shapes = {N_SHAPES}");

        let mut njets: u64 = 0;
        let mut nshapes: u64 = 0;
        for shape in RockShape::iter().cycle() {
            let mut rock = shape.projectile(rock_pile.tallest_point());
            rock.display();
            nshapes += 1;
            //println!("{:?}:{:?}", shape, rock.shape());
            //rock.display();
            loop {
                let jet = jet_iter.next();
                nshapes += 1;
                //print!("move {:?}", jet);
                match *jet.unwrap() {
                    JetDirection::Left => {
                        if rock.can_move_left() {
                            rock.move_left();
                            println!("left");
                            rock.display();
                        }
                    }
                    JetDirection::Right => {
                        if rock.can_move_right() {
                            rock.move_right();
                            println!("right");
                            rock.display();
                        }
                    }
                    _ => println!("panic"),
                }
                //print!("move {j}");
                //rock.display();
                if rock.can_move_down() {
                    rock.move_down();
                } else {
                    //println!("break loop");
                    //rock_pile.add(&mut rock);
                    break;
                }
            }

            n = n + 1;
            if n == NUM_ROCKS {
                break;
            }
        }
        println!("top {:?}", rock_pile.tallest_point());

        Ok(())
    }
}
