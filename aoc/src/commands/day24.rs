use std::path::PathBuf;

use clap::Parser;

use glam::IVec2;

use super::{CommandImpl, DynError};

use std::fmt;

use std::fs;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    *,
};

#[derive(Parser, Debug)]
pub struct Day24 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    South,
    East,
    West,
    North,
}

impl Direction {
    fn displace(&self) -> IVec2 {
        match self {
            Direction::West => IVec2::new(0, -1),
            Direction::East => IVec2::new(0, 1),
            Direction::North => IVec2::new(-1, 0),
            Direction::South => IVec2::new(1, 0),
        }
    }

    fn symbol(&self) -> char {
        match self {
            Direction::West => '<',
            Direction::East => '>',
            Direction::North => '^',
            Direction::South => 'v',
        }
    }
}

const COMPASS: &'static [Direction; 4] =
    &[Direction::South, Direction::East, Direction::North, Direction::West];

#[derive(Debug, Clone, PartialEq)]
struct Expedition {
    location: IVec2,
    dimension: IVec2,
}

impl Expedition {
    fn new(location: IVec2, dimension: IVec2) -> Expedition {
        Expedition { location, dimension }
    }

    fn move_to(&self, direction: Direction) -> Expedition {
        //println!("move_to({:?})", direction);
        let location = self.location + direction.displace();
        let dimension = self.dimension.clone();
        Expedition { location, dimension }
    }

    fn is_legal(&self) -> bool {
        //println!("is_legal({:?})", self.location);
        let move_result: bool = self.location[0] >= 0
            && self.location[0] < self.dimension[0]
            && self.location[1] >= 0
            && self.location[1] < self.dimension[1];
        //println!("is_legal({:?}) = {move_result}", self.location);
        move_result
    }

    fn possible_moves(&self) -> Vec<Expedition> {
        COMPASS.iter().map(|d| self.move_to(*d)).filter(|d| d.is_legal()).collect()
    }

    fn destination_reached(&self) -> bool {
        let dim = self.dimension.to_array();
        let destination: IVec2 = IVec2::new(dim[0] - 1, dim[1] - 1);
        self.location == destination
    }
}

impl fmt::Display for Expedition {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.location)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Blizzard {
    location: IVec2,
    direction: Direction,
}

impl Blizzard {
    fn new(location: IVec2, direction: Direction) -> Blizzard {
        Blizzard { location, direction }
    }

    fn displace(&mut self) {
        self.location = self.location + self.direction.displace();
    }

    fn modulo(&mut self, pos: usize, modulo: i32) {
        let mut values = self.location.to_array();
        values[pos] = values[pos].rem_euclid(modulo);
        self.location = IVec2::new(values[0], values[1]);
    }

    fn is_blizzard(&self, location: IVec2) -> bool {
        location == self.location
    }
}

fn parse_valley(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, vecs) = separated_list1(newline, many1(one_of(".#><^v")))(input)?;
    Ok((input, vecs))
}

fn get_empty_valley(length: usize, width: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; width]; length]
}

// this is likely an unecessary step
fn remove_walls(vecs: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (length, width) = get_valley_dimension(vecs);
    let mut valley = get_empty_valley(length, width);
    for h in 1..(vecs.len() - 1) {
        for w in 1..(vecs[0].len() - 1) {
            valley[h - 1][w - 1] = vecs[h][w];
        }
    }
    valley
}

fn blizzard(index: usize, direction: Direction, length: i32, width: i32) -> Option<Blizzard> {
    let index: i32 = index as i32;
    let location: IVec2 = IVec2::new(index / width, index.rem_euclid(width));
    println!("blizzard({index}): {:?}", location);
    Some(Blizzard::new(location, direction))
}

fn locate_blizzards(vecs: &Vec<Vec<char>>) -> Vec<Blizzard> {
    let width: i32 = vecs[0].len() as i32;
    let length: i32 = vecs.len() as i32;
    vecs.iter()
        .flatten()
        .enumerate()
        .filter_map(|b| match b.1 {
            '>' => blizzard(b.0, Direction::East, length, width),
            '<' => blizzard(b.0, Direction::West, length, width),
            'v' => blizzard(b.0, Direction::South, length, width),
            '^' => blizzard(b.0, Direction::North, length, width),
            _ => None,
        })
        .collect()
}

#[derive(Debug)]
struct Valley {
    blizzards: Vec<Blizzard>,
    expedition: Expedition,
    destination: IVec2,
    counter: u32,
}

impl Valley {
    fn new(blizzards: Vec<Blizzard>, length: usize, width: usize) -> Valley {
        let length = length as i32;
        let width = width as i32;
        let destination: IVec2 = IVec2::new(length, width);
        let location: IVec2 = IVec2::new(-1, 0);
        let expedition: Expedition = Expedition::new(location, destination);
        let counter: u32 = 0;
        Valley { blizzards, expedition, destination, counter }
    }

    fn score(&self, location: IVec2) -> i32 {
        let curr = location.to_array();
        let dim = self.destination.to_array();

        if self.is_blizzard(location) || curr[0] < 0 || curr[1] < 0 {
            -1000
        } else {
            dim[0] + dim[1] - ((dim[0] - curr[0]) + (dim[1] - curr[1]))
        }
    }

    fn minute(&mut self) {
        let dim = self.destination.to_array();

        for b in self.blizzards.iter_mut() {
            b.displace();
            b.modulo(0, dim[0]);
            b.modulo(1, dim[1]);
        }
        let mut best_score: i32 = self.score(self.expedition.location);
        for new_move in self.expedition.possible_moves() {
            if self.score(new_move.location) > best_score {
                self.expedition = new_move;
                best_score = self.score(self.expedition.location);
            }
        }
        println!("Minute {:?}, {:?}", self.counter, self.expedition.location);
        self.counter += 1;
    }

    fn is_blizzard(&self, p: IVec2) -> bool {
        for b in self.blizzards.iter() {
            if p == b.location {
                return true;
            }
        }

        false
    }

    fn symbol(&self, p: IVec2) -> String {
        if p == self.expedition.location {
            return String::from("E");
        }
        for b in self.blizzards.iter() {
            if p == b.location {
                return String::from(b.direction.symbol());
            }
        }

        String::from(".")
    }

    fn destination_reached(&self) -> bool {
        self.expedition.destination_reached()
    }
}

impl fmt::Display for Valley {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Expedition: {}\n", self.expedition));
        let dim = self.destination.to_array();
        for w in 0..dim[0] {
            for h in 0..dim[1] {
                let p: IVec2 = IVec2::new(w, h);
                output.push_str(&format!("{}", self.symbol(p)));
            }
            output.push_str(&format!("\n"));
        }
        write!(f, "{}", output)
    }
}

fn get_valley_dimension(vecs: &Vec<Vec<char>>) -> (usize, usize) {
    let (length, width) = (vecs.len() - 2, vecs[0].len() - 2);
    (length, width)
}

impl CommandImpl for Day24 {
    fn main(&self) -> Result<(), DynError> {
        let characters = fs::read_to_string(&self.input).unwrap();
        let (_, vecs) = parse_valley(&characters).unwrap();
        println!("parse_valley");
        for v in vecs.iter() {
            println!("{:?}", v);
        }
        let valley: Vec<Vec<char>> = remove_walls(&vecs);
        println!("parse_valley");
        for v in valley.iter() {
            println!("{:?}", v);
        }
        let mut blizzards = locate_blizzards(&valley);
        let mut valley: Valley = Valley::new(blizzards, valley.len(), valley[0].len());

        while !valley.destination_reached() {
            valley.minute();
            println!("{valley}");
        }
        println!("reached in {:?} minutes", valley.counter);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_blizzard_north() {
        let direction: Direction = Direction::North;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        let expected: IVec2 = IVec2::new(1, 1);
        assert_eq!(expected, blizzard.expedition);

        blizzard.displace();
        let expected: IVec2 = IVec2::new(0, 1);
        assert_eq!(expected, blizzard.expedition);
    }

    #[test]
    fn move_blizzard_south() {
        let direction: Direction = Direction::South;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(2, 1);
        assert_eq!(expected, blizzard.expedition);
    }

    #[test]
    fn move_blizzard_east() {
        let direction: Direction = Direction::East;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(1, 2);
        assert_eq!(expected, blizzard.expedition);
    }

    #[test]
    fn move_blizzard_west() {
        let direction: Direction = Direction::East;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(1, 2);
        assert_eq!(expected, blizzard.expedition);
    }
}
