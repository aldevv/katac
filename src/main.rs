use crate::args::Args;
use clap::Parser;
use katac::{copy_kata, create_day, get_dst_path, get_random_katas, run_make_command};

pub mod args;

fn main() {
    env_logger::init();
    let args = Args::parse();
    if args.run.is_none() {
        if !args.kata_names.is_empty() && args.random.is_some() {
            println!("You can't specify both specific katas and random katas.");
            return;
        }

        let mut kata_names: Vec<String> = Vec::new();
        if args.random.is_some() {
            kata_names = get_random_katas(args.random.clone(), args.katas_dir.clone())
        }

        if kata_names.is_empty() {
            kata_names = args.kata_names.clone();
        }

        create_day();
        copy_kata(kata_names, args.katas_dir, args.days_dir);
        return;
    }

    match args.run.unwrap() {
        args::Run::Run { kata_names } => {
            for (i, kata_name) in kata_names.iter().enumerate() {
                let path = format!("{}/{}", get_dst_path(args.days_dir.clone()), kata_name);
                let makefile_path = format!("{}/Makefile", path);
                if !std::path::Path::new(&makefile_path).exists() {
                    println!("No Makefile found in {}", path);
                    continue;
                }

                println!(
                    "\n> Running {} [{}/{}]\n_______________________",
                    kata_name,
                    i + 1,
                    kata_names.len()
                );
                let mut child = run_make_command(kata_name.to_string(), path);
                let code = child.wait().expect("failed to wait on child");
                assert!(code.success());
            }
        }
    }
}
