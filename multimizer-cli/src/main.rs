pub mod cli;
pub mod codon_usage;
pub mod utils;

use clap::Parser;

use cli::Cli;
use std::io;

use crate::codon_usage::pull_codon_usage_for_org;

fn main() {
    let cli = Cli::parse();

    let verbose = cli.verbose;
    let debug = cli.debug;

    match cli.cmd {
        Some(cli::Commands::Optimize { query }) => {
            let query = match query {
                Some(q) => q,
                None => {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read from stdin");
                    input.trim().to_string()
                }
            };
            println!("Optimizing {query}");
        }
        Some(cli::Commands::Pull { id }) => {
            let codon_usage = pull_codon_usage_for_org(id).expect("Failed to pull codon usage");
            println!("{}", codon_usage);
        }
        None => unreachable!(),
    }
}
