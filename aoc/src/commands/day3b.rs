use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day3b {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut badges: Vec<u8> = Vec::new();
        let mut score: usize = 0;
        let mut i: usize = 0;

        for line in lines {
            println!("Round {} Rucksack :{}", i, line);
            let mut rucksack: Vec<u8> = line.into_bytes();
            rucksack.sort();
            rucksack.dedup();

            if (i % 3) == 0 {
                badges = rucksack.clone();
                println!("round {} first badges length :{}", i, badges.len());
            } else {
                badges = intersect(badges, &rucksack);
                if badges.is_empty() {
                    println!("broke:{:?} {:?}", badges, rucksack);
                }
                println!("round {} intersect length :{}", i, badges.len());
                if (i % 3) == 2 {
                    score += get_item_priority(badges[0]) as usize;
                    badges.clear();
                }
            }
            i += 1;
        }

        println!("Total Score:{}", score);
        Ok(())
    }
}

/// Convert ASCII value to value
///
/// A-Z are 65-90 which map to 27-52
/// a-z are priority 1-26
///
#[inline]
fn get_item_priority(byte: u8) -> u8 {
    if byte <= 90 {
        byte - 38
    } else {
        byte - 96
    }
}

pub fn intersect(this: Vec<u8>, second: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut duplicate: Vec<u8> = Vec::new();

    'outer: for x in this {
        if duplicate.contains(&x) {
            break 'outer;
        }
        duplicate.push(x);
        for y in second {
            if x == *y {
                result.push(x);
            }
        }
    }
    result
}
