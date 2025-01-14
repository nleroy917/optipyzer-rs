use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long, default_value_t=false)]
    pub debug: bool,

    #[command(subcommand)]
    pub cmd: Option<Commands>,

}


#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum Commands {
    /// Optimize a query sequence
    Optimize {
        #[arg(required=false, value_name="QUERY")]
        /// The query sequence to optimize
        query: Option<String>,
    },

    /// Pull usage data from the database
    Pull {
        #[arg(required=true, value_name="ORGANISM")]
        id: i32,
    }
}