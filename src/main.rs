use std::process;

use katac::{
    args::{Args, Subcommands::Create, Subcommands::Random, Subcommands::Run},
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
        None => katac.select(),
        Some(ref subcommand) => match subcommand {
            Run {
                kata_names,
                command,
            } => katac.run(kata_names.clone(), command.clone()),
            Create { kata_name } => katac.create(kata_name.to_string()),
            Random { number_of_katas } => katac.random_katas(*number_of_katas),
        },
    }
}
