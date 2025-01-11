use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Optimize a query sequence
    Optimize {
        #[arg(required=true, value_name="QUERY")]
        query: String,
    },

    /// Pull usage data from the database
    Pull {
        #[arg(required=true, value_name="ORGANISM")]
        id: i32,
    }
}