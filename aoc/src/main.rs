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
    Day8(day8::Day8),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}
