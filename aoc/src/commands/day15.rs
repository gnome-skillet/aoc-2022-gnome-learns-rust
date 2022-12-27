use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut, RangeInclusive};
//use std::{collections::VecDeque, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, preceded},
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
        let mut covered: Vec<Point> = vec![];
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
        BeaconExclusionZone((start..=end))
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
    this: &mut BeaconExclusionZone,
    that: &mut BeaconExclusionZone,
) -> BeaconExclusionZone {
    BeaconExclusionZone::new(*this.start().min(that.start()), *this.end().max(that.end()))
}

impl CommandImpl for Day15 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut existing_beacons: HashSet<i32> = HashSet::new();
        let mut ranges: Vec<BeaconExclusionZone> = vec![];
        let row: i32 = 2;
        let mut min_x: i32 = i32::MAX;
        let mut max_x: i32 = i32::MIN;

        for line in lines {
            let x = read_line(&line);
            let (sensor, beacon) = x.unwrap().1;
            let smallest_x = sensor.x.min(beacon.x);
            min_x = min_x.min(smallest_x);
            let largest_x = sensor.x.max(beacon.x);
            max_x = max_x.max(largest_x);
            if sensor.distance(&beacon) >= sensor.perpendicular_distance(row) {
                ranges.push(sensor.coverage(&beacon, row).unwrap());
            }
            // keep tally of existing beacons on row
            if beacon.perpendicular_distance(row) == 0 {
                existing_beacons.insert(beacon.x);
            }
        }
        ranges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ranges.reverse();
        let mut result: Vec<BeaconExclusionZone> = vec![];
        let mut item: BeaconExclusionZone = ranges.pop().unwrap();
        while !ranges.is_empty() {
            let mut top: BeaconExclusionZone = ranges.pop().unwrap();
            println!("compare {:?} with {:?}", item, top);
            if item.overlaps(&top) {
                item = item.coalesce(&top);
                println!("coalesce {:?}", item);
            } else {
                //result.push(item);
            }
        }
        //result.push(item);
        println!("evaluate item ({:?})", item);
        println!("evaluate ranges ({:?})", ranges);
        //vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("row length ({min_x}, {max_x})");
        let grid_range: RangeInclusive<i32> = min_x..=max_x;

        println!("Range of row({row}) {:?}", grid_range);
        println!("Covers {:?} grid before correction", existing_beacons.len());
        println!("existing beacon {:?}", existing_beacons);
        //6275922
        //println!("EX: {:?}", self.input);
        Ok(())
    }
}
