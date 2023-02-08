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
    x: i64,
    y: i64,
}

const UPPER_LIMIT: i64 = 4000000;

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    pub fn coordinates(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    // taxicab distance between 2 points
    pub fn distance(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    // vertical distance from a point to a line
    pub fn perpendicular_distance(&self, row: i64) -> u64 {
        self.y.abs_diff(row)
    }

    // range of points corresponding to exclusion zone for a row
    pub fn coverage(&self, beacon: &Self, row: i64) -> Option<ExclusionZone> {
        let perp_distance: u64 = self.perpendicular_distance(row);
        let zero: i64 = 0;
        let ceiling: i64 = UPPER_LIMIT;
        if perp_distance > self.distance(beacon) {
            return None;
        }
        let width: i64 = (self.distance(beacon) - perp_distance) as i64;

        Some(ExclusionZone::new(zero.max(self.x - width), ceiling.min(self.x + width)))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Eq, Debug, Clone)]
pub struct ExclusionZone(RangeInclusive<i64>);

impl ExclusionZone {
    fn new(start: i64, end: i64) -> ExclusionZone {
        ExclusionZone(start..=end)
    }

    // determine if 2 ranges overlap
    pub fn overlaps(&self, other: &Self) -> bool {
        other.0.start() <= self.0.end() && other.0.start() >= self.0.start()
            || other.0.end() <= self.0.end() && other.0.end() >= self.0.start() ||
            other.0.start() - self.0.end() == 1 ||
            self.0.start() - other.0.end() == 1
    }

    // coalesce two overlapping ranges into a larger range
    pub fn coalesce(&self, other: &Self) -> ExclusionZone {
        let self_start: i64 = *self.0.start();
        let other_start: i64 = *other.0.start();

        let self_end: i64 = *self.0.end();
        let other_end: i64 = *other.0.end();
        ExclusionZone::new(self_start.min(other_start), self_end.max(other_end))
    }
}

impl Deref for ExclusionZone {
    type Target = RangeInclusive<i64>;
    fn deref(&self) -> &RangeInclusive<i64> {
        &self.0
    }
}

impl DerefMut for ExclusionZone {
    fn deref_mut(&mut self) -> &mut RangeInclusive<i64> {
        &mut self.0
    }
}

impl Ord for ExclusionZone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.start().cmp(&other.0.start())
    }
}

impl PartialOrd for ExclusionZone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExclusionZone {
    fn eq(&self, other: &Self) -> bool {
        self.0.start() == other.0.start()
    }
}

fn read_line(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, sensorx) = preceded(tag("Sensor at x="), nom::character::complete::i64)(input)?;
    let (input, sensory) = preceded(tag(", y="), nom::character::complete::i64)(input)?;
    let (input, beaconx) =
        preceded(tag(": closest beacon is at x="), nom::character::complete::i64)(input)?;
    let (input, beacony) = preceded(tag(", y="), nom::character::complete::i64)(input)?;
    let sensor: Point = Point::new(sensorx, sensory);
    let beacon: Point = Point::new(beaconx, beacony);
    Ok((input, (sensor, beacon)))
}

pub fn coalesce_range(
    mut ranges: Vec<ExclusionZone>,
) -> Vec<ExclusionZone> {
        let mut result: Vec<ExclusionZone> = vec![];
        let mut item: ExclusionZone = ranges.pop().unwrap();
        while !ranges.is_empty() {
            let top: ExclusionZone = ranges.pop().unwrap();
            //println!("compare {:?} with {:?}", item, top);
            if item.overlaps(&top) {
                item = item.coalesce(&top);
                //println!("coalesce {:?}", item);
            } else {
                result.push(item);
                item = top;
            }
        }
        result.push(item);
        result
}

pub fn excluded_positions(ranges: &Vec<ExclusionZone>) -> u64 {
    let mut n: u64 = 0;
    for r in ranges {
        let x: u64 =  (r.end() - r.start() + 1) as u64;
        //println!("exclude {:?} to {:?}", r.start(), r.end());
        n += x;
    }
    n
}
pub struct SearchSpace {
    sensors_beacons: Vec<(Point, Point)>,
    grid_range: RangeInclusive<i64>,
}

impl SearchSpace {
    fn new(lines: Vec<String>) -> SearchSpace {
        let mut min_x: i64 = i64::MAX;
        let mut max_x: i64 = i64::MIN;
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
        let grid_range: RangeInclusive<i64> = min_x..=max_x;
        SearchSpace{ sensors_beacons, grid_range }
    }

    fn reachable_points(&self, row: i64) -> Vec<ExclusionZone> {
        let mut ranges: Vec<ExclusionZone> = vec![];
        for (sensor, beacon) in &self.sensors_beacons {
            if sensor.distance(&beacon) >= sensor.perpendicular_distance(row) {
                ranges.push(sensor.coverage(&beacon, row).unwrap());
            }
        }
        ranges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        ranges.reverse();
        let coalesced_ranges: Vec<ExclusionZone> = coalesce_range(ranges);
        coalesced_ranges
    }

    fn beacons_in_row(&self, row: i64) -> usize {
        let mut existing_beacons: HashSet<Point> = HashSet::new();
        for (_, beacon) in &self.sensors_beacons {
            // keep tally of existing beacons on row
            if beacon.perpendicular_distance(row) == 0 {
                existing_beacons.insert(*beacon);
            }
        }
        existing_beacons.len()
    }

    fn uncovered_points(&self, range: &Vec<ExclusionZone>) -> Vec<i64> {
        let mut points: Vec<i64> = vec![];

        for p in 0..(UPPER_LIMIT + 1) {
            let mut is_in_range = true;
            for coverage in range.iter() {
                if coverage.contains(&p) {
                    is_in_range = false;
                    break;
                } else {
                    continue;
                }
            }
            if is_in_range {
                points.push(p);
            }
        }
        points
    }

    fn coverage(&self, row: i64) -> i64 {
        let ranges: &Vec<ExclusionZone> = &self.reachable_points(row);
        let n_existing_beacons: u64 = self.beacons_in_row(row) as u64;
        let n_ranges: usize = ranges.len();
        let n_excluded_positions: u64 = excluded_positions(&ranges) - n_existing_beacons;
        let n_excluded_positions_corrected: u64 = n_excluded_positions - n_existing_beacons;
        //let n_total_positions: i64 = self.grid_range.end() - self.grid_range.start();
        //let n_total_positions: i64 = ranges[0].end() - ranges[0].start() + 1;
        if n_ranges > 1 {
          //println!("row {row} from {:?} to {:?}", self.grid_range.start(), self.grid_range.end());
          //println!("excluded positions {:?}", n_excluded_positions_corrected);
          let points: Vec<i64> = self.uncovered_points(ranges);
          for p in points {
              println!("score({p}, {row}) = {:?}", my_score(p, row));
          }
        }
        n_excluded_positions_corrected as i64
    }
}

fn my_score(x: i64, y: i64) -> i64 {
    x * 4000000 + y
}

impl CommandImpl for Day15 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let search_space: SearchSpace = SearchSpace::new(lines);
        for row in 0..(UPPER_LIMIT + 1) {
            let _n: i64 = search_space.coverage(row);
        }
        //6275922
        Ok(())
    }
}
