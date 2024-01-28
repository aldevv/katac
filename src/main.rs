use katac::{copy_katas, random_katas, run_katas, Args, Subcommands};

use clap::Parser;

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.subcommand {
        None => copy_katas(&args, &args.kata_names),
        Some(ref subcommand) => match subcommand {
            Subcommands::Run { kata_names } => run_katas(&args, kata_names),
            Subcommands::Random { number_of_katas } => {
                copy_katas(&args, &random_katas(&args, *number_of_katas))
            }
        },
    }
}
