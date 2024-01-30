use katac::{
    copy_katas, new_kata, random_katas, run_katas, Args, Subcommands::New, Subcommands::Random,
    Subcommands::Run,
};

use clap::Parser;

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.subcommand {
        None => copy_katas(&args, &args.kata_names),
        Some(ref subcommand) => match subcommand {
            Run {
                kata_names,
                command,
            } => run_katas(&args, kata_names.clone(), command.clone()),
            Random { number_of_katas } => copy_katas(&args, &random_katas(&args, *number_of_katas)),
            New { kata_name } => new_kata(&args, kata_name.to_string()),
        },
    }
}
