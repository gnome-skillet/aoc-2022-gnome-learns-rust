pub mod commands;
pub mod utils;

use commands::*;
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day0(day0::Day0),
    Day3(day3::Day3),
    Day3b(day3b::Day3b),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
