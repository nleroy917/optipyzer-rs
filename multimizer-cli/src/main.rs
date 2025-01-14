pub mod cli;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    let verbose = cli.verbose;
    let debug = cli.debug;

    println!("Verbose: {}", verbose);
    println!("Debug: {}", debug);
}
