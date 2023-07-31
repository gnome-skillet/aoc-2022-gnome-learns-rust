use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

#[derive(Parser, Debug)]
pub struct Day1a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day1a {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut meal_calories: u32 = 0; # total caloric value per meal
        let mut max_meal_calories: u32 = 0; # calories of highest calories meal
        let mut meals: Vec<u32> = Vec::new();

        for line in lines {
            if line.is_empty() {
                meals.push(meal_calories);
                if meal_calories > max_meal_calories {
                    max_meal_calories = meal_calories;
                }
                meal_calories = 0;
            } else {
                meal_calories += line.parse::<u32>().unwrap();
            }
        }
        println!("The Elf carrying the most calories is carrying {} calories",
                 max_meal_calories);
        meals.sort_by(|a, b| b.cmp(a));
        let sum_top_three: u32 = meals.iter().take(3).sum();
        println!("The top three Elves carrying the most calories are carrying {} calories",
                 sum_top_three);
        Ok(())
    }
}
