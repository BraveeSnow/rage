use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rage")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "I MISS THE RAGE")]
pub struct RageOptions {
    #[command(subcommand)]
    pub command: RageCommands,

    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum RageCommands {
    /// Sort USE flags located in package.use
    Sort(RageSortOptions),
}

#[derive(Args, Debug)]
pub struct RageSortOptions {
    #[arg(short, long = "pretend", default_value_t = false)]
    #[arg(help = "Write results to stdout instead of USE files")]
    pub pretend_mode: bool,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "\x1b[31m*DESTRUCTIVE* Removes all versioning in package.use\x1b[30m")]
    pub remove_versions: bool,
    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Sorts package atoms into files named after their category")]
    pub split: bool,
}
