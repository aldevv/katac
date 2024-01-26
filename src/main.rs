use crate::args::{copy_kata, Args};
use clap::Parser;
use katac::{create_day, get_dst_path, run_make_command};

pub mod args;

fn main() {
    env_logger::init();
    let args = Args::parse();

    // TODO: detect that a day has not passed so create the katas in the same folder(?)
    // TODO: generate random katas for today from the katas folder

    if args.run.is_none() {
        create_day();
        copy_kata(args);
        return;
    }

    match args.run.unwrap() {
        args::Run::Run { kata_names } => {
            for (i, kata_name) in kata_names.iter().enumerate() {
                println!(
                    "\n> Running {} [{}/{}]\n_______________________",
                    kata_name,
                    i + 1,
                    kata_names.len()
                );
                let path = format!("{}/{}", get_dst_path(args.days_dir.clone()), kata_name);
                let mut child = run_make_command(kata_name.to_string(), path);
                let code = child.wait().expect("failed to wait on child");
                assert!(code.success());
            }
        }
    }
}
