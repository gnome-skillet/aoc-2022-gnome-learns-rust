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
    fn can_move_down(&self, chamber: &Space) -> bool;
    fn can_move_left(&self, chamber: &Space) -> bool;
    fn can_move_right(&self, chamber: &Space) -> bool;
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn come_to_rest(&self, chamber: &mut Space);
    fn leftmost_point(&self) -> Option<u64>;
    fn rightmost_point(&self) -> Option<u64>;
    fn lowest_point(&self) -> Option<u64>;
    fn highest_point(&self) -> Option<u64>;
    fn shape(&self) -> &'static str;
    fn display(&self);
}

#[derive(Debug)]
pub struct Rock {
    points: Vec<(u64, u64)>,
}

struct OccupiedSpaces {
    points: Vec<(u64, u64)>,
}

pub fn plank(start: (u64, u64)) -> Rock {
    //println!("plank({:?})", start);
    Rock {
        points: vec![
            (start.0 + 0, start.1),
            (start.0 + 1, start.1),
            (start.0 + 2, start.1),
            (start.0 + 3, start.1),
        ],
    }
}

pub fn cross(start: (u64, u64)) -> Rock {
    //println!("cross({:?})", start);
    Rock {
        points: vec![
            (start.0 + 1, start.1),
            (start.0 + 0, start.1 + 1),
            (start.0 + 1, start.1 + 1),
            (start.0 + 2, start.1 + 1),
            (start.0 + 1, start.1 + 2),
        ],
    }
}

pub fn ell(start: (u64, u64)) -> Rock {
    //println!("ell({:?})", start);
    Rock {
        points: vec![
            (start.0 + 0, start.1),
            (start.0 + 1, start.1),
            (start.0 + 2, start.1),
            (start.0 + 2, start.1 + 1),
            (start.0 + 2, start.1 + 2),
        ],
    }
}

pub fn pole(start: (u64, u64)) -> Rock {
    //println!("pole({:?})", start);
    Rock { points: (0..4).map(|y| (start.0, start.1 + y)).collect() }
}

pub fn block(start: (u64, u64)) -> Rock {
    //println!("block({:?})", start);
    Rock {
        points: vec![
            (start.0 + 0, start.1),
            (start.0 + 1, start.1),
            (start.0 + 0, start.1 + 1),
            (start.0 + 1, start.1 + 1),
        ],
    }
}

impl Projectile for Rock {
    fn move_down(&mut self) {
        self.points = self.points.iter().map(|(x, y)| (*x, y - 1)).collect::<Vec<(u64, u64)>>();
    }

    fn move_left(&mut self) {
        self.points = self.points.iter().map(|(x, y)| (x - 1, *y)).collect::<Vec<(u64, u64)>>();
    }

    fn move_right(&mut self) {
        self.points = self.points.iter().map(|(x, y)| (x + 1, *y)).collect::<Vec<(u64, u64)>>();
    }

    fn can_move_down(&self, chamber: &Space) -> bool {
        let lowest_point = self.lowest_point().unwrap();
        !(lowest_point == 0
            || self.points.iter().any(|p| chamber.occupied_space.contains(&(p.0, p.1 - 1))))
    }

    fn can_move_left(&self, chamber: &Space) -> bool {
        !(self.leftmost_point() == Some(0)
            || self.points.iter().any(|p| chamber.occupied_space.contains(&(p.0 - 1, p.1))))
    }

    fn can_move_right(&self, chamber: &Space) -> bool {
        !(self.rightmost_point() == Some(6)
            || self.points.iter().any(|p| chamber.occupied_space.contains(&(p.0 + 1, p.1))))
    }

    fn leftmost_point(&self) -> Option<u64> {
        self.points.iter().map(|(x, _)| x).min().copied()
    }

    fn rightmost_point(&self) -> Option<u64> {
        self.points.iter().map(|(x, _)| x).max().copied()
    }

    fn highest_point(&self) -> Option<u64> {
        self.points.iter().map(|(_, y)| y).max().copied()
    }

    fn lowest_point(&self) -> Option<u64> {
        self.points.iter().map(|(_, y)| y).min().copied()
    }

    fn shape(&self) -> &'static str {
        //format!("rock {:?}", self.points).as_str()
        "rock"
    }

    fn display(&self) {
        println!("{:?}", self.points);
    }

    fn come_to_rest(&self, chamber: &mut Space) {
        chamber.occupied_space.extend(self.points.iter().cloned());
        chamber.starting_position = (chamber.starting_position.0, chamber.top().unwrap() + 4);
    }
}

#[derive(Debug)]
pub struct Space {
    occupied_space: HashSet<(u64, u64)>,
    starting_position: (u64, u64),
}

impl Space {
    fn new() -> Space {
        Space {
            occupied_space: HashSet::from([
                (0u64, 0u64),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
            ]),
            starting_position: (2, 4),
        }
    }

    fn top(&self) -> Option<u64> {
        self.occupied_space.iter().map(|(_, y)| y).max().copied()
    }

    fn is_open(&self, rock: &Rock) -> bool {
        !rock.points.iter().any(|p| self.occupied_space.contains(p))
    }

    fn display(&self) {
        for y in (0..self.top().unwrap() + 1).rev() {
            for x in 0..7 {
                let point: (u64, u64) = (x, y);
                //print!("{:?}", point);
                if self.occupied_space.contains(&point) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn is_covered(&mut self, row: u64) -> bool {
        (0..7).into_iter().map(|x| (x, row)).all(|p| self.occupied_space.contains(&p))
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
    pub fn projectile(&self, start: (u64, u64)) -> Box<dyn Projectile> {
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

const NUM_ROCKS: u64 = 2022;
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
        let mut chamber = Space::new();
        //println!("Chamber: {:?}", chamber);
        let N_JETS: u64 = jets.len() as u64;
        const N_SHAPES: u64 = 5;
        println!("njets = {N_JETS}, n_shapes = {N_SHAPES}");

        let mut njets: u64 = 0;
        let mut nshapes: u64 = 0;
        for shape in RockShape::iter().cycle() {
            let mut rock = shape.projectile(chamber.starting_position);
            nshapes += 1;
            //println!("{:?}:{:?}", shape, rock.shape());
            //rock.display();
            loop {
                let jet = jet_iter.next();
                nshapes += 1;
                //print!("move {:?}", jet);
                match *jet.unwrap() {
                    JetDirection::Left => {
                        if rock.can_move_left(&chamber) {
                            rock.move_left();
                        }
                    }
                    JetDirection::Right => {
                        if rock.can_move_right(&chamber) {
                            rock.move_right();
                        }
                    }
                    _ => println!("panic"),
                }
                //print!("move {j}");
                //rock.display();
                if rock.can_move_down(&chamber) {
                    rock.move_down();
                } else {
                    //println!("break loop");
                    rock.come_to_rest(&mut chamber);
                    break;
                }
            }

            if nshapes.rem_euclid(N_SHAPES) == 0
                && njets.rem_euclid(N_JETS) == 0
                && chamber.is_covered(chamber.top().unwrap())
            {
                println!("cycle(nshapes = {nshapes}, njets = {njets})");
            }
            //println!("loop broken");
            //println!("{:?}", chamber);
            //chamber.display();
            n = n + 1;
            if n == NUM_ROCKS {
                break;
            }
        }
        println!("top {:?}", chamber.top());

        Ok(())
    }
}
