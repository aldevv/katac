use katac::{
    copy_katas, init_from_examples, new_kata, random_katas, run_katas, upgrade_katac, Args,
    Subcommands::Init, Subcommands::New, Subcommands::Random, Subcommands::Run,
    Subcommands::Upgrade,
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
            } => run_katas(&args, kata_names, command),
            Random { number_of_katas } => copy_katas(&args, &random_katas(&args, *number_of_katas)),
            New { kata_name } => new_kata(&args, kata_name),
            Init {
                examples_dir,
                select,
            } => init_from_examples(&args, examples_dir, select),
            Upgrade { force } => upgrade_katac(*force),
        },
    }
}
