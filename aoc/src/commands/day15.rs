use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use std::collections::HashSet;

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut, RangeInclusive};
//use std::{collections::VecDeque, fmt::Display};

use nom::{
    bytes::complete::tag,
    sequence::{preceded},
    *,
};

#[derive(Parser, Debug)]
pub struct Day15 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Hash, Eq, Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn coordinates(&self) -> (i32, i32) {
        let (x, y) = (self.x, self.y);
        (x, y)
    }

    pub fn distance(&self, other: &Self) -> u32 {
        let xdistance: u32 = self.x.abs_diff(other.x);
        let ydistance: u32 = self.y.abs_diff(other.y);
        xdistance + ydistance
    }

    pub fn perpendicular_distance(&self, row: i32) -> u32 {
        self.y.abs_diff(row)
    }

    pub fn coverage(&self, beacon: &Self, row: i32) -> Option<BeaconExclusionZone> {
        let perp_distance: u32 = self.perpendicular_distance(row);
        if perp_distance > self.distance(beacon) {
            return None;
        }
        let width: i32 = (self.distance(beacon) - perp_distance) as i32;

        Some(BeaconExclusionZone::new(self.x - width, self.x + width))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Eq, Debug, Clone)]
pub struct BeaconExclusionZone(RangeInclusive<i32>);

impl BeaconExclusionZone {
    fn new(start: i32, end: i32) -> BeaconExclusionZone {
        BeaconExclusionZone(start..=end)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        other.0.start() <= self.0.end() && other.0.start() >= self.0.start()
            || other.0.end() <= self.0.end() && other.0.end() >= self.0.start()
    }

    pub fn coalesce(&self, other: &Self) -> BeaconExclusionZone {
        let self_start: i32 = *self.0.start();
        let other_start: i32 = *other.0.start();

        let self_end: i32 = *self.0.end();
        let other_end: i32 = *other.0.end();
        BeaconExclusionZone::new(self_start.min(other_start), self_end.max(other_end))
    }
}

impl Deref for BeaconExclusionZone {
    type Target = RangeInclusive<i32>;
    fn deref(&self) -> &RangeInclusive<i32> {
        &self.0
    }
}

impl DerefMut for BeaconExclusionZone {
    fn deref_mut(&mut self) -> &mut RangeInclusive<i32> {
        &mut self.0
    }
}

impl Ord for BeaconExclusionZone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.start().cmp(&other.0.start())
    }
}

impl PartialOrd for BeaconExclusionZone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BeaconExclusionZone {
    fn eq(&self, other: &Self) -> bool {
        self.0.start() == other.0.start()
    }
}

fn read_line(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, sensorx) = preceded(tag("Sensor at x="), nom::character::complete::i32)(input)?;
    let (input, sensory) = preceded(tag(", y="), nom::character::complete::i32)(input)?;
    let (input, beaconx) =
        preceded(tag(": closest beacon is at x="), nom::character::complete::i32)(input)?;
    let (input, beacony) = preceded(tag(", y="), nom::character::complete::i32)(input)?;
    let sensor: Point = Point::new(sensorx, sensory);
    let beacon: Point = Point::new(beaconx, beacony);
    Ok((input, (sensor, beacon)))
}

pub fn coalesce_range(
    mut ranges: Vec<BeaconExclusionZone>,
) -> Vec<BeaconExclusionZone> {
        let mut result: Vec<BeaconExclusionZone> = vec![];
        let mut item: BeaconExclusionZone = ranges.pop().unwrap();
        while !ranges.is_empty() {
            let top: BeaconExclusionZone = ranges.pop().unwrap();
            //println!("compare {:?} with {:?}", item, top);
            if item.overlaps(&top) {
                item = item.coalesce(&top);
                //println!("coalesce {:?}", item);
            } else {
                result.push(top);
            }
        }
        result.push(item);
        result
}

pub struct SearchSpace {
    sensors_beacons: Vec<(Point, Point)>,
    grid_range: RangeInclusive<i32>,
}

impl SearchSpace {
    fn new(lines: Vec<String>) -> SearchSpace {
        let mut min_x: i32 = i32::MAX;
        let mut max_x: i32 = i32::MIN;
        let mut sensors_beacons: Vec<(Point, Point)> = vec![];
        for line in lines {
            let x = read_line(&line);
            let (sensor, beacon) = x.unwrap().1;
            let smallest_x = sensor.x.min(beacon.x);
            min_x = min_x.min(smallest_x);
            let largest_x = sensor.x.max(beacon.x);
            max_x = max_x.max(largest_x);
            sensors_beacons.push((sensor, beacon));
        }
        let grid_range: RangeInclusive<i32> = min_x..=max_x;
        SearchSpace{ sensors_beacons, grid_range }
    }

    fn coverage(&self, row: i32) -> i32 {
        //let row: i32 = 2000000;
        //let row: i32 = 10;
        let mut existing_beacons: HashSet<Point> = HashSet::new();
        let mut ranges: Vec<BeaconExclusionZone> = vec![];
        for (sensor, beacon) in &self.sensors_beacons {
            if sensor.distance(&beacon) >= sensor.perpendicular_distance(row) {
                ranges.push(sensor.coverage(&beacon, row).unwrap());
            }
            // keep tally of existing beacons on row
            if beacon.perpendicular_distance(row) == 0 {
                existing_beacons.insert(*beacon);
            }
        }
        ranges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ranges.reverse();
        let ranges = coalesce_range(ranges);
        let n_existing_beacons: i32 = existing_beacons.len() as i32;
        let n_excluded_positions: i32 = ranges[0].end() - ranges[0].start() + 1 - n_existing_beacons;
        let n_total_positions: i32 = self.grid_range.end() - self.grid_range.start();
        println!("excluded positions {:?} out of {:?}", n_excluded_positions, n_total_positions);
        n_excluded_positions
    }
}

impl CommandImpl for Day15 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        //let row: i32 = 2000000;
        let row: i32 = 10;

        let search_space: SearchSpace = SearchSpace::new(lines);
        let n: i32 = search_space.coverage(row);
        println!("there are {n} excluded positions");
        //6275922
        Ok(())
    }
}
