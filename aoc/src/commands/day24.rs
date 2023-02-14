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
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn displace(&self) -> IVec2 {
        match self {
            Direction::Left => IVec2::new(0, -1),
            Direction::Right => IVec2::new(0, 1),
            Direction::Up => IVec2::new(-1, 0),
            Direction::Down => IVec2::new(1, 0),
        }
    }

    fn symbol(&self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Blizzard {
    position: IVec2,
    direction: Direction,
}

impl Blizzard {
    fn new(position: IVec2, direction: Direction) -> Blizzard {
        Blizzard { position, direction }
    }

    fn displace(&mut self) {
        self.position = self.position + self.direction.displace();
    }

    fn set(&mut self, pos: usize, value: i32) {
        let mut values = self.position.to_array();
        values[pos] = value;
        self.position = IVec2::new(values[0], values[1]);
    }

    fn curr_position(&self) -> IVec2 {
        self.position
    }

    fn is_blizzard(&self, position: IVec2) -> bool {
        position == self.position
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
    println!("remove_walls({:?})", valley);
    valley
}

fn blizzard(index: usize, direction: Direction, length: i32, width: i32) -> Option<Blizzard> {
    let index: i32 = index as i32;
    let position: IVec2 = IVec2::new(index / length, index.rem_euclid(width));
    println!("blizzard: index={index}, length={length}, width={width}: {:?}", position);
    Some(Blizzard::new(position, direction))
}

fn locate_blizzards(vecs: &Vec<Vec<char>>) -> Vec<Blizzard> {
    let width: i32 = vecs[0].len() as i32;
    let length: i32 = vecs.len() as i32;
    println!("locate_blizzards: length={length}, width={width}");
    vecs.iter()
        .flatten()
        .enumerate()
        .filter_map(|b| match b.1 {
            '>' => blizzard(b.0, Direction::Right, width, length),
            '<' => blizzard(b.0, Direction::Left, width, length),
            'v' => blizzard(b.0, Direction::Down, width, length),
            '^' => blizzard(b.0, Direction::Up, width, length),
            _ => None,
        })
        .collect()
}

#[derive(Debug)]
struct Valley {
    blizzards: Vec<Blizzard>,
    position: IVec2,
    destination: IVec2,
}

impl Valley {
    fn new(blizzards: Vec<Blizzard>, length: usize, width: usize) -> Valley {
        let mut position: IVec2 = IVec2::new(-1, 0);
        let length = length as i32;
        let width = width as i32;
        let destination: IVec2 = IVec2::new(width + 1, length);
        Valley { blizzards, position, destination }
    }

    fn minute(&mut self) {
        let dim = self.destination.to_array();
        println!("dim {:?}", dim);

        println!("destination: {:?}", self.destination);
        for b in self.blizzards.iter_mut() {
            b.displace();
            let array = b.position.to_array();
            if array[0] == -1 {
                b.set(0, dim[0] - 1);
            } else if array[0] == (dim[0] - 1) {
                b.set(0, 0);
            } else if array[1] == -1 {
                b.set(1, dim[1] - 2);
            } else if array[1] == dim[1] {
                b.set(1, 0);
            }
            println!("displace {:?}", b);
        }
    }

    fn is_blizzard(&self, p: IVec2) -> bool {
        for b in self.blizzards.iter() {
            if p == b.position {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for Valley {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let dim = self.destination.to_array();
        for w in 0..(dim[0] - 1) {
            for h in 0..dim[1] {
                let p: IVec2 = IVec2::new(w, h);
                if self.is_blizzard(p) {
                    output.push_str(&format!(">"));
                } else {
                    output.push_str(&format!("."));
                }
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
        println!("{:?}", vecs);
        let valley: Vec<Vec<char>> = remove_walls(&vecs);
        let mut blizzards = locate_blizzards(&valley);
        let mut valley: Valley = Valley::new(blizzards, valley.len(), valley[0].len());
        println!("\n");
        println!("{valley}");
        println!("\n");
        valley.minute();
        println!("{valley}");
        valley.minute();
        println!("{valley}");
        valley.minute();
        println!("{valley}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_blizzard() {
        let direction: Direction = Direction::Up;
        let position: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(position, direction);
        let expected: IVec2 = IVec2::new(1, 1);
        assert_eq!(expected, blizzard.position);

        blizzard.displace();
        let expected: IVec2 = IVec2::new(1, 0);
        assert_eq!(expected, blizzard.position);

        let direction: Direction = Direction::Down;
        let position: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(position, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(1, 2);
        assert_eq!(expected, blizzard.position);

        let direction: Direction = Direction::Left;
        let position: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(position, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(0, 1);
        assert_eq!(expected, blizzard.position);

        let direction: Direction = Direction::Right;
        let position: IVec2 = IVec2::new(1, 1);
        let mut blizzard: Blizzard = Blizzard::new(position, direction);
        blizzard.displace();
        let expected: IVec2 = IVec2::new(2, 1);
        assert_eq!(expected, blizzard.position);
    }
}
