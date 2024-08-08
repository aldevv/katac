use std::process;

use katac::{
    args::{Args, Subcommands::New, Subcommands::Random, Subcommands::Run},
    Katac,
};

use clap::Parser;

fn main() {
    env_logger::init();
    let args = Args::try_parse().unwrap_or_else(|err| {
        let _ = err.print();
        process::exit(2);
    });

    let mut katac = Katac::new(&args);

    match args.subcommand {
        None => katac.save_and_copy_prompt(),
        Some(ref subcommand) => match subcommand {
            Run {
                kata_names,
                command,
            } => katac.run_katas(kata_names.clone(), command.clone()),
            // TODO: fix random
            Random { number_of_katas } => katac.copy_katas(&katac.random_katas(*number_of_katas)),
            New { kata_name } => katac.new_kata(kata_name.to_string()),
        },
    }
}
