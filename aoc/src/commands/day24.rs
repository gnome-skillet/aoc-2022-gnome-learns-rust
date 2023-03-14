use clap::Parser;

use glam::IVec2;

use super::{CommandImpl, DynError};

use std::collections::HashSet;

use std::path::PathBuf;

use std::mem::swap;

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
    Stationary,
}

impl Direction {
    fn displace(&self) -> IVec2 {
        match self {
            Direction::West => IVec2::new(0, -1),
            Direction::East => IVec2::new(0, 1),
            Direction::North => IVec2::new(-1, 0),
            Direction::South => IVec2::new(1, 0),
            Direction::Stationary => IVec2::new(0, 0),
        }
    }

    fn symbol(&self) -> char {
        match self {
            Direction::South => 'v',
            Direction::East => '>',
            Direction::Stationary => '.',
            Direction::North => '^',
            Direction::West => '<',
        }
    }
}

const COMPASS: &'static [Direction; 5] =
    &[Direction::South, Direction::East, Direction::Stationary, Direction::North, Direction::West];

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

    //fn is_blizzard(&self, location: IVec2) -> bool {
    //    location == self.location
    //}
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
    //println!("blizzard({index}): {:?}", location);
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
    start_location: IVec2,
    destination: IVec2,
    legal_moves: HashSet<IVec2>,
    length: usize,
    width: usize,
    minute: u32,
    ntrips: usize,
}

impl Valley {
    fn new(blizzards: Vec<Blizzard>, length: usize, width: usize) -> Valley {
        let l: i32 = (length - 1) as i32;
        let w: i32 = (width - 1) as i32;
        let destination: IVec2 = IVec2::new(l, w);
        let start_location: IVec2 = IVec2::new(-1, 0);
        let mut legal_moves: HashSet<IVec2> = HashSet::new();
        legal_moves.insert(start_location);
        let minute: u32 = 0;
        let ntrips: usize = 0;
        Valley {
            blizzards,
            start_location,
            destination,
            legal_moves,
            length,
            width,
            minute,
            ntrips,
        }
    }

    fn reset(&mut self) {
        self.legal_moves.clear();
        if self.ntrips.rem_euclid(2) == 1 {
            let start_location: IVec2 = IVec2::new(self.length as i32, (self.width - 1) as i32);
            let destination: IVec2 = IVec2::new(0 as i32, 0 as i32);
            self.start_location = start_location;
            self.legal_moves.insert(start_location);
            self.destination = destination;
        } else {
            let start_location: IVec2 = IVec2::new(-1 as i32, 0 as i32);
            let destination: IVec2 = IVec2::new((self.length - 1) as i32, (self.width - 1) as i32);
            self.start_location = start_location;
            self.legal_moves.insert(start_location);
            self.destination = destination;
        }
        println!(
            "reset: start_location({:?}), destination({:?})",
            self.start_location, self.destination
        );
    }

    fn is_legal(&self, location: &IVec2) -> bool {
        let loc = location.to_array();
        let length: i32 = self.length as i32;
        let width: i32 = self.width as i32;
        loc[0] >= 0 && loc[0] < length && loc[1] >= 0 && loc[1] < width
    }

    fn is_blizzard(&self, p: &IVec2) -> bool {
        for b in self.blizzards.iter() {
            if *p == b.location {
                return true;
            }
        }

        false
    }

    fn show_blizzards(&self) {
        let dim = self.destination.to_array();
        println!("dimension: {:?}", dim);
        print!("#");
        for _ in 0..self.width {
            print!("#");
        }
        print!("#");
        println!("");
        for i in 0..self.length {
            print!("#");
            for j in 0..self.width {
                let p: IVec2 = IVec2::new(i as i32, j as i32);
                if self.is_blizzard(&p) {
                    print!("B");
                } else if self.legal_moves.contains(&p) {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!("#");
        }
        print!("#");
        for _ in 0..self.width {
            print!("#");
        }
        print!("#");
        println!("");
    }

    fn is_location_legal(&self, location: &IVec2) -> bool {
        !self.is_blizzard(location) && self.is_legal(location)
    }

    fn destination_reached(&self) -> bool {
        self.legal_moves.contains(&self.destination)
    }

    fn possible_moves(&self, curr: &IVec2) -> Vec<IVec2> {
        COMPASS.iter().map(|d| *curr + d.displace()).filter(|d| self.is_legal(d)).collect()
    }

    fn move_blizzards(&mut self) {
        for b in self.blizzards.iter_mut() {
            b.displace();
            b.modulo(0, self.length as i32);
            b.modulo(1, self.width as i32);
        }
    }

    fn reachable_locations(&mut self, current_position: &IVec2) -> Vec<IVec2> {
        let mut locations: Vec<IVec2> = vec![];
        for new_move in self.possible_moves(current_position) {
            if self.is_location_legal(&new_move) {
                locations.push(new_move);
            }
        }
        if locations.len() == 0 {
            //println!("no legal moves");
            let start_location: IVec2 = self.start_location;
            locations.push(start_location);
        }
        locations
    }

    fn visit_future_locations(&mut self) {
        let mut legal_moves: HashSet<IVec2> = HashSet::new();
        swap(&mut legal_moves, &mut self.legal_moves);
        let positions: Vec<IVec2> =
            legal_moves.into_iter().flat_map(|x| self.reachable_locations(&x)).collect();
        self.legal_moves = HashSet::from_iter(positions.iter().cloned())
    }

    fn simulate_minute(&mut self) {
        self.move_blizzards();
        self.visit_future_locations();
        self.minute += 1;
    }

    fn simulate(&mut self) -> u32 {
        while !self.destination_reached() {
            self.simulate_minute();
        }
        self.move_blizzards();
        self.ntrips += 1;
        self.minute += 1;
        self.reset();
        self.minute
    }
}

impl fmt::Display for Valley {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("Expedition: {}\n", self.start_location));
        //let dim = self.destination.to_array();
        //for w in 0..dim[0] {
        //    for h in 0..dim[1] {
        //        let p: IVec2 = IVec2::new(w, h);
        //        output.push_str(&format!("{}", self.symbol(p)));
        //    }
        //    output.push_str(&format!("\n"));
        //}
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
        let valley: Vec<Vec<char>> = remove_walls(&vecs);
        let mut blizzards = locate_blizzards(&valley);
        let mut valley: Valley = Valley::new(blizzards, valley.len(), valley[0].len());
        let nminutes: u32 = valley.simulate();
        println!("reached in {:?} minutes", nminutes);
        let nminutes: u32 = valley.simulate();
        println!("reached in {:?} minutes", nminutes);
        let nminutes: u32 = valley.simulate();
        println!("reached in {:?} minutes", nminutes);

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
        assert_eq!(expected, blizzard.location);

        blizzard.displace();
        let expected: IVec2 = IVec2::new(0, 1);
        assert_eq!(expected, blizzard.location);
    }

    #[test]
    fn move_blizzard_south() {
        let direction: Direction = Direction::South;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(2, 1);
        assert_eq!(expected, blizzard.location);
    }

    #[test]
    fn move_blizzard_east() {
        let direction: Direction = Direction::East;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(1, 2);
        assert_eq!(expected, blizzard.location);
    }

    #[test]
    fn move_blizzard_west() {
        let direction: Direction = Direction::East;
        let expedition: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(expedition, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(1, 2);
        assert_eq!(expected, blizzard.location);
    }
}
