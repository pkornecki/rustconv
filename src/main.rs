mod cli;

use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

use cli::CommandLineArgs;

#[tokio::main]
async fn main() {
    // get command line arguments
    let CommandLineArgs { input, output, hotels } = CommandLineArgs::from_args();

    let input = input
        .or_else(|| Some(PathBuf::from("input.csv")))
        .expect("Failed to find input.csv");

    let output = output
        .or_else(|| Some(PathBuf::from("output.csv")))
        .expect("Failed to find output.csv");

    let hotels = hotels
        .or_else(|| Some(PathBuf::from("hotels.json")))
        .expect("Failed to find hotels.json");

    // run the conversion and report any potential errors
    if let Err(e) = rustconv::run(input, output, hotels).await {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
