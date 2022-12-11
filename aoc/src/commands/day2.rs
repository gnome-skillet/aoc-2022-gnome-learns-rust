use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use regex::Regex;

use std::str::FromStr;

use strum_macros::EnumString;

use std::fmt;

#[derive(Parser, Debug)]
pub struct Day2 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Copy, Clone, EnumString)]
pub enum Move {
    #[strum(
        serialize = "A",
        serialize = "X",
        serialize = "A Y",
        serialize = "B X",
        serialize = "C Z"
    )]
    Rock = 1,
    #[strum(
        serialize = "B",
        serialize = "Y",
        serialize = "A Z",
        serialize = "B Y",
        serialize = "C X"
    )]
    Paper = 2,
    #[strum(
        serialize = "C",
        serialize = "Z",
        serialize = "A X",
        serialize = "B Z",
        serialize = "C Y"
    )]
    Scissors = 3,
}

fn score_round(m: &Move, o: &Outcome) -> u32 {
    *m as u32 + *o as u32
}

#[derive(Copy, Clone, EnumString, Debug)]
pub enum Outcome {
    #[strum(serialize = "A Z", serialize = "B X", serialize = "C Y", serialize = "X")]
    Loss = 0,
    #[strum(serialize = "A X", serialize = "B Y", serialize = "C Z", serialize = "Y")]
    Draw = 3,
    #[strum(serialize = "A Y", serialize = "B Z", serialize = "C X", serialize = "Z")]
    Win = 6,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissors => write!(f, "Scissors"),
        }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Outcome::Loss => write!(f, "Loss"),
            Outcome::Draw => write!(f, "Draw"),
            Outcome::Win => write!(f, "Win"),
        }
    }
}

impl CommandImpl for Day2 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut strategy_1: u32 = 0;
        let mut strategy_2: u32 = 0;
        let mut i: usize = 1;
        for line in lines {
            i += 1;
            if line.is_empty() {
                continue;
            }
            let re = Regex::new(r"^([ABC])\s*([XYZ])\s*$").unwrap();
            let cap = re.captures(&line).unwrap();
            let protagonist_move = Move::from_str(&cap[2]).unwrap();
            let antagonist_move = Move::from_str(&cap[1]).unwrap();
            let protagonist_countermove = Move::from_str(&cap[0]).unwrap();

            let outcome_1 = Outcome::from_str(&cap[0]).unwrap();
            let outcome_2 = Outcome::from_str(&cap[2]).unwrap();
            let score_1: u32 = score_round(&protagonist_move, &outcome_1);
            let score_2: u32 = score_round(&protagonist_countermove, &outcome_2);
            strategy_1 += score_1;
            strategy_2 += score_2;
            println!(
                "(Day2a({line}): Move ({antagonist_move}, {protagonist_move}, {outcome_1}) = {score_1}"
            );
            println!(
                "Day2b({line}): Move ({antagonist_move}, {protagonist_countermove}, {outcome_2}) = {score_2}."
                );
        }

        println!("Match Score Day2a: {:?}", strategy_1);
        println!("Match Score Day2b: {:?}", strategy_2);
        Ok(())
    }
}
