use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

//use std::collections::HashMap;

//use std::collections::HashSet;

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

#[derive(Debug)]
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

    fn right(&self) -> Point {
        let x = self.x + 1;
        let y = self.y;
        Point::new(x, y)
    }

    fn left(&self) -> Point {
        let x = self.x - 1;
        let y = self.y;
        Point::new(x, y)
    }

    fn up(&self) -> Point {
        let x = self.x;
        let y = self.y + 1;
        Point::new(x, y)
    }

    fn down(&self) -> Point {
        let x = self.x;
        let y = self.y - 1;
        Point::new(x, y)
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
pub struct Knot {
    head: Vec<Point>,
    tail: Vec<Point>,
    name: String,
}

impl Knot {
    fn new(name: String) -> Knot {
        let mut p: Point = Point::origin();
        let mut head: Vec<Point> = vec![];
        head.push(p);
        let mut p: Point = Point::origin();
        let mut tail: Vec<Point> = vec![];
        tail.push(p);
        Knot { head, tail, name }
    }

    fn move_head(&mut self, movement: Move) {
        println!("command: {:?}", movement);
        let (x, y, z): (i32, i32, u8) = match movement {
            Move::Right(s) => (1, 0, s),
            Move::Left(s) => (-1, 0, s),
            Move::Up(s) => (0, 1, s),
            Move::Down(s) => (0, -1, s),
            Move::UpRight(s) => (1, 1, s),
            Move::DownRight(s) => (1, -1, s),
            Move::UpLeft(s) => (-1, 1, s),
            Move::DownLeft(s) => (-1, -1, s),
            Move::None => (0, 0, 0),
        };
        for i in 0..z {
            let top = self.head.pop().unwrap();
            let p: Point = Point::new(top.x + x, top.y + y);
            println!("move {:?} -> {:?}", top, p);
            self.head.push(top);
            self.head.push(p);
        }
    }

    fn trace_tail(&mut self) {
        let mut tail_index: usize = 0;
        let mut head_index: usize = 1;
        let best_move: Move = self.tail[tail_index].best_move(&self.head[head_index]);
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
        let mut knot: Knot = Knot::new("head".to_string());
        let head: Point = Point::new(2, 2);
        let tail: Point = Point::origin();
        let best_move = tail.best_move(&head);
        println!("best move {:?} -> {:?} is {:?}", tail, head, best_move);

        //let newmove = read_move("L 4");
        //println!("{:?}", newmove);
        for line in lines {
            let knot_movement = read_move(&line);
            let movement = knot_movement.unwrap().1;
            knot.move_head(movement);
        }
        knot.trace_tail();
        println!("{:?}", knot);
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
