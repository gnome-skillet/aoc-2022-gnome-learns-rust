use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    // character::complete::multispace1,
    //number::complete::u8,
    sequence::preceded,
    *,
};

#[derive(Parser, Debug)]
pub struct Day9 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub enum Move {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
    UpRight(u8),
    DownRight(u8),
    UpLeft(u8),
    DownLeft(u8),
    None,
}

impl Move {
    fn deconstruct_move(&self) -> (i32, i32, u8) {
        match *self {
            Move::Right(s) => (1, 0, s),
            Move::Left(s) => (-1, 0, s),
            Move::Up(s) => (0, 1, s),
            Move::Down(s) => (0, -1, s),
            Move::UpRight(s) => (1, 1, s),
            Move::DownRight(s) => (1, -1, s),
            Move::UpLeft(s) => (-1, 1, s),
            Move::DownLeft(s) => (-1, -1, s),
            Move::None => (0, 0, 0),
        }
    }
}

#[derive(Debug, Hash, Eq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn coordinates(&self) -> (i32, i32) {
        let (x, y) = (self.x, self.y);
        (x, y)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let xdistance: f32 = self.x.abs_diff(other.x) as f32;
        let ydistance: f32 = self.y.abs_diff(other.y) as f32;
        let distance = xdistance.powf(2.0) + ydistance.powf(2.0);
        distance.powf(0.5)
    }

    pub fn best_move(&self, other: &Self) -> Move {
        let distance: f32 = self.distance(other) as f32;
        if distance.abs() <= 1.5 {
            return Move::None;
        }
        let number: i32 = self.x - other.x;
        let xdiff: i32 = match number {
            -2 | -1 => 1,
            2 | 1 => -1,
            _ => 0,
        };

        let number: i32 = self.y - other.y;
        let ydiff: i32 = match number {
            -2 | -1 => 1,
            2 | 1 => -1,
            _ => 0,
        };

        match (xdiff, ydiff) {
            (1, 0) => Move::Right(1),
            (-1, 0) => Move::Left(1),
            (1, 1) => Move::UpRight(1),
            (-1, 1) => Move::UpLeft(1),
            (0, 1) => Move::Up(1),
            (0, -1) => Move::Down(1),
            (1, -1) => Move::DownRight(1),
            (-1, -1) => Move::DownLeft(1),
            _ => Move::None,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Vec<Point>>,
}

impl Rope {
    fn new(n_knots: usize) -> Rope {
        let mut knots: Vec<Vec<Point>> = vec![vec![]; n_knots + 1];
        for i in 0..knots.len() {
            let p: Point = Point::origin();
            knots[i].push(p);
        }
        Rope { knots }
    }

    fn move_head(&mut self, movement: Move) {
        println!("command: {:?}", movement);
        let (x, y, z): (i32, i32, u8) = movement.deconstruct_move();
        for i in 0..z {
            let top = self.knots[0].pop().unwrap();
            let p: Point = Point::new(top.x + x, top.y + y);
            //println!("move {:?} -> {:?}", top, p);
            self.knots[0].push(top);
            self.knots[0].push(p);
        }
    }

    fn trace_tail(&mut self) {
        for j in 1..self.knots.len() {
            let mut tail_index: usize = 0;
            let mut head_index: usize = 1;
            for i in 1..self.knots[0].len() {
                let best_move: Move = self.knots[j][i - 1].best_move(&self.knots[j - 1][i]);
                let (x, y, z) = best_move.deconstruct_move();
                let p: Point = Point::new(self.knots[j][i - 1].x + x, self.knots[j][i - 1].y + y);
                self.knots[j].push(p);
            }
        }
    }

    fn n_unique_spaces(&self, index: usize) -> usize {
        let mut unique_values: HashSet<Point> = HashSet::new();
        for v in self.knots[index].iter() {
            let mut x: Point = v.clone();
            unique_values.insert(x);
        }
        unique_values.len()
    }

    fn n_visits_per_knot(&self) {
        for i in 0..self.knots.len() {
            println!("knots[{i}] = {:?}", self.knots[i].len());
        }
    }
}

fn read_move(input: &str) -> IResult<&str, Move> {
    let (input, direction) = alt((tag("R"), tag("L"), tag("U"), tag("D")))(input)?;
    let (input, steps) = preceded(tag(" "), nom::character::complete::u8)(input)?;

    let movement: Move = match direction.to_uppercase().as_str() {
        "R" => Move::Right(steps),
        "L" => Move::Left(steps),
        "U" => Move::Up(steps),
        "D" => Move::Down(steps),
        _ => unimplemented!("no other movements supported"),
    };

    Ok((input, movement))
}

impl CommandImpl for Day9 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut rope: Rope = Rope::new(10);

        //let newmove = read_move("L 4");
        //println!("{:?}", newmove);
        for line in lines {
            let knot_movement = read_move(&line);
            let movement = knot_movement.unwrap().1;
            rope.move_head(movement);
        }
        rope.trace_tail();
        rope.n_visits_per_knot();
        let mut unique_values: HashSet<Point> = HashSet::new();
        for v in rope.knots[1].iter() {
            let x: Point = v.clone();
            unique_values.insert(x);
        }
        for i in 1..11 {
            println!("knot[{i}] visted {:?} unique locations", rope.n_unique_spaces(i));
        }
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
