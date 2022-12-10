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
        let mut caloric_total: u32 = 0;
        let mut max_calories: u32 = 0;
        let mut sum_top_three: u32 = 0;
        let mut snacks: Vec<u32> = Vec::new();

        for line in lines {
            if line.is_empty() {
                let x = caloric_total;
                snacks.push(x);
                if caloric_total > max_calories {
                    max_calories = caloric_total;
                }
                caloric_total = 0;
            } else {
                caloric_total += line.parse::<u32>().unwrap();
            }
        }
        snacks.sort_by(|a, b| b.cmp(a));
        println!("Max calories {}", max_calories);
        for n in 0..3 {
            let snack_cals = snacks[n];
            println!("Snack cals {}", snack_cals);
            sum_top_three += snack_cals;
        }
        println!("Sum calories of top 3: {}", sum_top_three);
        Ok(())
    }
}
