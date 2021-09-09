use std::path::PathBuf;
use structopt::StructOpt;

/// A structure holding command line arguments.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "rustconv",
    about = "rustconv is a program written in Rust, that converts input file(`input.csv`) into output file(`output.csv`).
Some data which is necessary for the converion is taken from the `hotels.json` file."
)]
pub struct CommandLineArgs {
    /// Use a different input file.
    #[structopt(parse(from_os_str), short, long)]
    pub input: Option<PathBuf>,

    /// Use a different output file.
    #[structopt(parse(from_os_str), short, long)]
    pub output: Option<PathBuf>,

    /// Use a different hotels file.
    #[structopt(parse(from_os_str), short, long)]
    pub hotels: Option<PathBuf>,
}
