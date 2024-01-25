use clap::{Parser, Subcommand};
use katac::{get_curday, get_dst, get_src};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    katas_dir: Option<String>,

    #[arg(short, long)]
    pub days_dir: Option<String>,

    #[command(subcommand)]
    pub run: Option<Run>,

    #[arg(num_args = 1..)]
    pub kata_names: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Run {
    /// run a kata
    Run {
        /// kata to run
        #[arg(required = true, num_args = 1)]
        kata_name: String,
    },
}

pub fn copy_kata(args: Args) {
    let copy_options = fs_extra::dir::CopyOptions::new();
    for kata_name in &args.kata_names {
        let src = get_src(kata_name, args.katas_dir.clone());
        let dst = get_dst(args.days_dir.clone());
        match fs_extra::copy_items(&[src], dst, &copy_options) {
            Ok(_) => println!("Copying {} to day{}...", kata_name, get_curday()),
            Err(e) => println!("Error: {}", e),
        }
    }
}
