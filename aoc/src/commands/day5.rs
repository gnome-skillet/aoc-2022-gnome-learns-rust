use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use regex::Regex;

#[derive(Parser, Debug)]
pub struct Day5 {
    #[clap(long, short)]
    input: PathBuf,
}

pub struct Crane {}

#[derive(Debug)]
pub struct Port {
    stacks: Vec<Vec<char>>,
    n_crates: usize,
}

impl CommandImpl for Day5 {
    fn main(&self) -> Result<(), DynError> {
        let mut stacks = vec![];
        let lines: Vec<String> = slurp_file(&self.input)?;
        let n_crates: usize = lines[0].len() / 4 + 1;
        let offset: usize = 4;
        let mut start_commands: usize = 0;

        // set up empty crates
        for _i in 0..n_crates {
            let mut x: Vec<char> = vec![];
            stacks.push(x);
        }
        let re = Regex::new(r"^\[([A-Z])\]\s*$").unwrap();
        let re2 = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

        // load items onto crates
        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                start_commands = i;
                break;
            }
            for crate_no in 0..n_crates {
                let start: usize = crate_no * offset;
                let stop: usize =
                    if (start + offset) < line.len() { start + offset } else { line.len() };
                let slice = &line[start..stop];
                if re.is_match(&slice) {
                    //println!("Slice {}", slice);
                    let cap = re.captures(&slice).unwrap();
                    let item: char = cap[1].parse().unwrap();
                    stacks[crate_no].push(item);
                }
            }
        }
        for crate_no in 0..n_crates {
            stacks[crate_no].reverse();
        }

        println!("Before: {:?}", stacks);
        for i in start_commands..lines.len() {
            if re2.is_match(&lines[i]) {
                let cap = re2.captures(&lines[i]).unwrap();
                let n_items: usize = cap[1].parse().unwrap();
                let from: usize = cap[2].parse().unwrap();
                let to: usize = cap[3].parse().unwrap();
                println!("Move {n_items} from {from} to {to}");
                for j in 0..n_items {
                    let x = stacks[from - 1].pop().unwrap();
                    stacks[to - 1].push(x);
                }
            }
        }
        println!("After: {:?}", stacks);
        let mut s = String::new();
        for i in 0..n_crates {
            s.push(stacks[i].pop().unwrap());
        }

        println!("Top of stacks {s}");
        // guessed SHQWSRBD
        // SHQWSRBDL
        //println!("There are {} crates", stacks.len());
        //println!("Start Commands at line {start_commands}");
        Ok(())
    }
}
