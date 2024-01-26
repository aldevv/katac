use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
/// Katac is a tool to help you do katas everyday
pub struct Args {
    /// Custom directory to copy katas from (default: ./katas)
    #[arg(short, long)]
    pub katas_dir: Option<String>,

    /// Custom directory to copy katas to everyday (default: ./days)
    #[arg(short, long)]
    pub days_dir: Option<String>,

    /// Number of katas you want to do today, randomly taken from katas.toml
    #[arg(short, long)]
    pub random: Option<u8>,

    #[command(subcommand)]
    pub run: Option<Run>,

    /// Katas you want to do today
    #[arg(num_args = 1..)]
    pub kata_names: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Run {
    /// Katas you want to run today (requires a makefile with the  'run' target in the kata's root folder)
    Run {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        kata_names: Vec<String>,
    },
}
