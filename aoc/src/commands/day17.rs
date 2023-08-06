use camino::Utf8PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

#[derive(Parser, Debug)]
pub struct Day17 {
    #[clap(long, short)]
    input: Utf8PathBuf,
}

enum JetDirection {
    Right,
    Left,
}

#[derive(Debug)]
pub struct Rock {
    points: Vec<(u32, u32)>,
}

struct OccupiedSpaces {
    points: Vec<(u32, u32)>,
}

pub fn plank(height: u32) -> Rock {
    Rock { points: vec![(0, height), (1, height), (2, height), (3, height)] }
}

pub fn cross(height: u32) -> Rock {
    Rock {
        points: vec![
            (1, height),
            (0, height + 1),
            (1, height + 1),
            (2, height + 2),
            (2, height + 1),
        ],
    }
}

pub fn ell(height: u32) -> Rock {
    Rock { points: vec![(0, height), (1, height), (2, height), (2, height + 1), (2, height + 2)] }
}

pub fn pole(height: u32) -> Rock {
    Rock { points: (0..4).map(|x| (x, height)).collect() }
}

pub fn block(height: u32) -> Rock {
    Rock { points: vec![(0, height), (1, height), (0, height + 1), (1, height + 1)] }
}

impl Rock {
    fn move_down(&self) -> Option<Self> {
        Some(Rock {
            points: self.points.iter().map(|(x, y)| (*x, y - 1)).collect::<Vec<(u32, u32)>>(),
        })
    }

    fn move_left(&self) -> Option<Rock> {
        Some(Rock {
            points: self.points.iter().map(|(x, y)| (x - 1, *y)).collect::<Vec<(u32, u32)>>(),
        })
    }

    fn move_right(&self) -> Option<Rock> {
        Some(Rock {
            points: self.points.iter().map(|(x, y)| (x + 1, *y)).collect::<Vec<(u32, u32)>>(),
        })
    }
}

pub enum RockShape {
    Plank,
    Cross,
    Ell,
    Pole,
    Block,
}

impl RockShape {
    pub fn set(&self, height: u32) -> Result<Rock, DynError> {
        match self {
            Plank => Ok(plank(height)),
            Cross => Ok(cross(height)),
            Ell => Ok(ell(height)),
            Pole => Ok(pole(height)),
            Block => Ok(block(height)),
        }
    }
}

impl CommandImpl for Day17 {
    fn main(&self) -> Result<(), DynError> {
        //let lines: Vec<String> = slurp_file(&self.input)?;
        //let jets: Vec<char> = something(lines);
        let block: Rock = block(0);
        let pole: Rock = pole(0);
        let ell: Rock = ell(0);
        let cross: Rock = cross(0);
        let plank: Rock = plank(0);

        println!("block {:?}", block);
        println!("pole {:?}", pole);
        println!("ell {:?}", ell);
        println!("cross {:?}", cross);
        println!("plank {:?}", plank);

        Ok(())
    }
}
