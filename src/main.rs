use crate::args::{copy_kata, Args};
use clap::Parser;
use katac::{create_day, get_dst_path, run_make_command};

pub mod args;

fn main() {
    env_logger::init();
    let args = Args::parse();

    if args.run.is_none() {
        if args.kata_names.is_empty() {
            // TODO: bug, when installing, it creates the days folder even tho no kata name is provided
            println!("{}", args.kata_names[0]);
            println!("No kata name provided");
            return;
        }
        create_day();
        copy_kata(args);
        return;
    }

    match args.run.unwrap() {
        args::Run::Run { kata_name } => {
            // run makefile command using "make" shell in the kata_name folder
            let path = format!("{}/{}", get_dst_path(args.days_dir.clone()), kata_name);
            run_make_command(kata_name, path)
        }
    }
}
