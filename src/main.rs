use std::process;

use katac::{
    args::{
        Args, KatacSubcommands::Add, KatacSubcommands::Random, KatacSubcommands::Run,
        KatacSubcommands::Workspace, WorkspaceSubcommands,
    },
    Katac,
};

use clap::Parser;
use log::debug;

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
            Add { kata_name } => katac.add(kata_name.to_string()),
            Random { number_of_katas } => katac.random_katas(*number_of_katas),
            Workspace { subcommand } => match subcommand {
                Some(subcommand) => match subcommand {
                    WorkspaceSubcommands::Add { name, path, remote } => {
                        debug!("Adding workspace: {} at path: {:?}", name, path);
                        katac.add_workspace(name, path, remote.clone())
                    }
                    WorkspaceSubcommands::List => {
                        debug!("Listing workspaces");
                    }
                    WorkspaceSubcommands::Remove { name } => {
                        debug!("Removing workspace: {}", name);
                    }

                    WorkspaceSubcommands::ListKatas { workspace_name } => {
                        debug!("Listing katas in workspace: {}", workspace_name);
                    }

                    WorkspaceSubcommands::ListAllKatas => {
                        debug!("Listing all katas");
                    }
                },
                None => {
                    println!("Not implemented yet");
                    process::exit(1);
                }
            },
        },
    }
}
