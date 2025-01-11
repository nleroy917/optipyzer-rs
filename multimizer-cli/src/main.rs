pub mod cli;

use clap::Parser;

use cli::Cli;

fn main() {
    let cli = Cli::parse();

    let verbose = cli.verbose;

    println!("Verbose: {}", verbose);
}
