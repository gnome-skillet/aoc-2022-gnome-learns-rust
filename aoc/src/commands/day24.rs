use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day24 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day24 {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
