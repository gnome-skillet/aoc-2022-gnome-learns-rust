use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use std::collections::HashSet;

#[derive(Parser, Debug)]
pub struct Day3 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day3 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut sum_priorities = 0;

        for line in lines {
          sum_priorities += identify_duplicate(&line);
        }
        println!("Total Score:{}", sum_priorities);
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
    if byte <= 90 { byte - 38} else { byte - 96}
}

fn identify_duplicate(s: &str) -> usize {
    let items = s.as_bytes();
    let len = s.len() / 2;
    let first_comp: HashSet<&u8> = HashSet::from_iter(&items[0..len]);
    let mut score = 0;

    for i in &items[len..] {
        if first_comp.contains(i) {
            score = get_item_priority(*i) as usize;
            break;
        }
    }

    score
}
