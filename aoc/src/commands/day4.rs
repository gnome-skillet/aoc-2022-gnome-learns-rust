use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

//use std::ops::Range;

use regex::Regex;

use std::ops::RangeInclusive;

#[derive(Parser, Debug)]
pub struct Day4 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct CleanupAssignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

fn build_cleanup_assignment(first: RangeInclusive<u32>, second: RangeInclusive<u32>) -> CleanupAssignment {
    CleanupAssignment {
        first,
        second,
    }
}

fn new_cleanup_assignment(lower1: u32, upper1: u32, lower2: u32, upper2: u32) -> CleanupAssignment {
    let first = RangeInclusive::new(lower1, upper1);
    let second = RangeInclusive::new(lower2, upper2);
    build_cleanup_assignment(first, second)
}

impl CleanupAssignment {
    pub fn has_overlap(&mut self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end())) ||
            (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    pub fn has_any_overlap(&mut self) -> bool {
        self.first.contains(self.second.start()) || self.first.contains(self.second.end()) ||
            self.second.contains(self.first.start()) || self.second.contains(self.first.end())
    }
}

impl CommandImpl for Day4 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut n_overlaps: usize = 0;
        let mut n_any_overlaps: usize = 0;

        for line in lines {
            let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
            let cap = re.captures(&line).unwrap();
            let lower1: u32 = cap[1].parse().unwrap();
            let upper1: u32 = cap[2].parse().unwrap();
            let lower2: u32 = cap[3].parse().unwrap();
            let upper2: u32 = cap[4].parse().unwrap();
            let mut assignment = new_cleanup_assignment(lower1, upper1, lower2, upper2);
            if assignment.has_overlap() {
                n_overlaps += 1;
            }
            if assignment.has_any_overlap() {
                n_any_overlaps += 1;
            }
        }

        println!("Number Overlaps: {n_overlaps}");
        println!("Number Any Overlaps: {n_any_overlaps}");
        Ok(())
    }
}
