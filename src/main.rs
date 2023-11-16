mod args;
mod commands;
mod portage;

use args::{RageCommands, RageOptions};
use clap::Parser;
use commands::sort::rage_command_sort;

fn main() {
    let opts: RageOptions = RageOptions::parse();

    match &opts.command {
        RageCommands::Sort(opts) => rage_command_sort(opts),
    }
}
