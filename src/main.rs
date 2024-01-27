use clap::{Parser, Subcommand};
use katac::{copy_kata, create_day, get_dst_path, get_random_katas, run_make_command};

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

    #[command(subcommand)]
    pub subcommand: Option<Subcommands>,

    /// Katas you want to do today
    #[arg(num_args = 1..)]
    pub kata_names: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Katas you want to run today (requires a makefile with the  'run' target in the kata's root folder)
    Run {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        kata_names: Vec<String>,
    },

    /// Number of katas you want to do today, randomly taken from katas.toml
    Random {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        number_of_katas: Option<u8>,
    },
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.subcommand {
        None => {
            create_day(args.days_dir.clone());
            copy_kata(args.kata_names, args.katas_dir, args.days_dir);
        }
        Some(subcommand) => match subcommand {
            Subcommands::Run { kata_names } => {
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

            Subcommands::Random { number_of_katas } => {
                let kata_names = get_random_katas(number_of_katas, args.katas_dir.clone());
                create_day(args.days_dir.clone());
                copy_kata(kata_names, args.katas_dir, args.days_dir);
            }
        },
    }
}
