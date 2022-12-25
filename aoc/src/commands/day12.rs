use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use std::collections::HashSet;

use std::collections::VecDeque;

#[derive(Parser, Debug)]
pub struct Day12 {
    #[clap(long, short)]
    input: PathBuf,
}

const ORIGIN_MARKER: u8 = 'S' as u8;
const DESTINATION_MARKER: u8 = 'E' as u8;
const ORIGIN_HEIGHT: u8 = 'a' as u8;
const DESTINATION_HEIGHT: u8 = 'z' as u8;

#[derive(Hash, Eq, Clone, Copy, Debug)]
pub struct Point {
    row: usize,
    column: usize,
    height: u8,
}

impl Point {
    fn new(row: usize, column: usize, height: u8) -> Point {
        let height = if height == DESTINATION_MARKER { DESTINATION_HEIGHT } else { height };
        let height = if height == ORIGIN_MARKER { ORIGIN_HEIGHT } else { height };
        Point { row, column, height }
    }

    pub fn has_edge(&self, other: &Self) -> bool {
        let reachable: bool = (other.height > self.height) || (self.height - other.height <= 1);
        let pos_diff: usize = self.row.abs_diff(other.row) + self.column.abs_diff(other.column);
        pos_diff <= 1 && reachable 
    }

    pub fn coordinates(&self) -> (usize, usize) {
        let (row, column) = (self.row, self.column);
        (row, column)
    }

    pub fn origin_point(row: usize, column: usize) -> Point {
        Point::new(row, column, ORIGIN_HEIGHT)
    }

    pub fn destination_point(row: usize, column: usize) -> Point {
        Point::new(row, column, DESTINATION_HEIGHT)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column && self.height == other.height
    }
}

#[derive(Debug)]
pub struct HeightMap {
    elevation: Vec<Vec<u8>>,
}

impl HeightMap {
    fn new(lines: Vec<String>) -> HeightMap {
        let mut elevation: Vec<Vec<u8>> = vec![];
        for line in lines {
            let s =
                line.chars().into_iter().filter(|&c| c.is_alphabetic()).map(|c| c as u8).collect();
            elevation.push(s);
        }
        HeightMap { elevation }
    }

    fn nrows(&self) -> usize {
        self.elevation.len()
    }

    fn ncols(&self) -> usize {
        self.elevation[0].len()
    }

    fn step_west(&self, current_position: Point) -> Option<Point> {
        let (r, c) = current_position.coordinates();
        if c == 0 {
            return None;
        }
        let left_step: Point = Point::new(r, c - 1, self.elevation[r][c - 1]);
        if !current_position.has_edge(&left_step) {
            return None;
        }
        Some(left_step)
    }

    fn step_east(&self, current_position: Point) -> Option<Point> {
        let (r, c) = current_position.coordinates();
        if c == (self.elevation[0].len() - 1) {
            return None;
        }
        let right_step: Point = Point::new(r, c + 1, self.elevation[r][c + 1]);
        if !current_position.has_edge(&right_step) {
            return None;
        }
        Some(right_step)
    }

    fn step_north(&self, current_position: Point) -> Option<Point> {
        let (r, c) = current_position.coordinates();
        if r == 0 {
            return None;
        }
        let up_step: Point = Point::new(r - 1, c, self.elevation[r - 1][c]);
        if !current_position.has_edge(&up_step) {
            return None;
        }
        Some(up_step)
    }

    fn step_south(&self, current_position: Point) -> Option<Point> {
        let (r, c) = current_position.coordinates();
        if r == (self.elevation.len() - 1) {
            return None;
        }
        let down_step: Point = Point::new(r + 1, c, self.elevation[r + 1][c]);
        if !current_position.has_edge(&down_step) {
            return None;
        }
        Some(down_step)
    }

    fn get_neighbors(&self, current_position: Point) -> Vec<Option<Point>> {
        let mut neighbors: Vec<Option<Point>> = vec![];
        neighbors.push(self.step_west(current_position));
        neighbors.push(self.step_east(current_position));
        neighbors.push(self.step_north(current_position));
        neighbors.push(self.step_south(current_position));
        neighbors
    }

    pub fn origin_location(&self) -> Option<Point> {
        self.find_marker(ORIGIN_MARKER)
    }

    pub fn destination_location(&self) -> Option<Point> {
        self.find_marker(DESTINATION_MARKER)
    }

    pub fn find_marker(&self, marker: u8) -> Option<Point> {
        let mut r: usize = 0;
        let marker_height: u8 =
            if marker == ORIGIN_MARKER { ORIGIN_HEIGHT } else { DESTINATION_HEIGHT };

        for row in self.elevation.iter() {
            let mut c: usize = 0;
            for col in row.iter() {
                if *col == marker {
                    return Some(Point::new(r, c, marker_height));
                }
                c += 1;
            }
            r += 1;
        }

        return None;
    }
}

#[derive(Debug)]
pub struct ShortestDistance {
    height_map: HeightMap,
    frontier: VecDeque<(Point, u32)>,
    visited: HashSet<Point>,
}

impl ShortestDistance {
    fn new(lines: Vec<String>) -> ShortestDistance {
        let height_map: HeightMap = HeightMap::new(lines);
        let origin: Point = height_map.origin_location().unwrap();
        let mut frontier = VecDeque::new();
        let mut visited: HashSet<Point> = HashSet::new();
        frontier.push_back((origin, 0));
        visited.insert(origin);
        ShortestDistance { height_map, frontier, visited }
    }

    fn display(&self) {
        println!("grid has {:?} rows", self.height_map.nrows());
        println!("grid has {:?} columns", self.height_map.ncols());
        println!("origin is at {:?}", self.height_map.origin_location());
        println!("destination is at {:?}", self.height_map.destination_location());
    }

    fn search(&mut self) -> Option<u32> {
        let destination: Point = self.height_map.destination_location().unwrap();
        let mut npushed: u32 = 0;
        let mut npopped: u32 = 0;
        while !self.frontier.is_empty() {
            println!("stack size {:?}", self.frontier.len());
            let (top, steps) = self.frontier.pop_front().unwrap();
            println!("{:?} is the parent with {steps} steps", top);
            npopped += 1;
            if destination == top {
                println!("destination found {:?} with {steps} steps", top);
                return Some(steps);
            }
            let mut nneighbors: u8 = 0;
            for n in self.height_map.get_neighbors(top) {
                match n {
                    Some(p) => {
                        if !self.visited.contains(&p) {
                            println!("\t\tvisit {:?}", p);
                            self.frontier.push_back((p, steps + 1));
                            self.visited.insert(p);
                            npushed += 1;
                            nneighbors += 1;
                        } else {
                            println!("\t\talready visited {:?}", p);
                        }
                    }
                    None => {
                        //println!("skip");
                    }
                }
            }
            println!("\t\t{:?} has {nneighbors} neighbors", top);
        }
        println!("n pushed {npushed}, npopped {npopped}");

        return None;
    }
}

impl CommandImpl for Day12 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        //let mut height_map: HeightMap = HeightMap::new(lines);
        let mut shortest_distance: ShortestDistance = ShortestDistance::new(lines);
        shortest_distance.display();
        let distance: Option<u32> = shortest_distance.search();

        println!("Distance: {:?}", distance);
        //println!("EX: {:?}", height_map.elevation);
        //println!("Origin: {:?}", height_map.origin_location());
        //println!("Destination: {:?}", height_map.destination_location());
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
